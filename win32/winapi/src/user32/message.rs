use std::{cell::RefCell, collections::VecDeque, rc::Rc, sync::LazyLock};

use runtime::Context;

use crate::{
    dllexport::win32flags,
    host, stub, trace,
    user32::{state, Window, HACCEL, HWND},
    Ptr, POINT,
};

/// If THESEUS_TRACE includes "wm", log all Windows messages.
static LOG_MESSAGES: LazyLock<bool> =
    LazyLock::new(|| !matches!(trace::get_uncached("wm"), trace::Trace::None));

pub type WPARAM = u32;
pub type LPARAM = u32;

#[derive(win32_derive::ABIEnum)]
pub enum WM {
    PAINT = 0xf,
    QUIT = 0x12,
    MOUSEMOVE = 0x200,
    LBUTTONDOWN = 0x201,
    LBUTTONUP = 0x202,
    RBUTTONDOWN = 0x204,
    RBUTTONUP = 0x205,
    MBUTTONDOWN = 0x207,
    MBUTTONUP = 0x208,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, zerocopy::FromBytes, zerocopy::IntoBytes, zerocopy::Immutable)]
pub struct MSG {
    hwnd: HWND,
    message: u32,
    wParam: WPARAM,
    lParam: LPARAM,
    time: u32,
    pt: POINT,
}

#[derive(Default)]
pub struct MessageQueue {
    pub window: Option<Rc<RefCell<Window>>>,
    messages: VecDeque<MSG>,
    quit: Option<MSG>,
}

win32flags! {
    pub struct MK {
        const LBUTTON = 0x0001;
        const RBUTTON = 0x0002;
        const SHIFT   = 0x0004;
        const CONTROL = 0x0008;
        const MBUTTON = 0x0010;
    }
}

fn mouse_button_to_wm(is_down: bool, message: &host::MouseMessage) -> WM {
    match (message.button, is_down) {
        (1, true) => WM::LBUTTONDOWN,
        (1, false) => WM::LBUTTONUP,
        (2, true) => WM::RBUTTONDOWN,
        (2, false) => WM::RBUTTONUP,
        (3, true) => WM::MBUTTONDOWN,
        (3, false) => WM::MBUTTONUP,
        _ => todo!("mouse button {}", message.button),
    }
}

fn mouse_msg(wm: WM, hwnd: HWND, message: &host::MouseMessage) -> MSG {
    let mut wParam = MK::empty();
    if message.buttons & 1 != 0 {
        wParam |= MK::LBUTTON;
    }
    if message.buttons & (1 << 1) != 0 {
        wParam |= MK::RBUTTON;
    }
    if message.buttons & (1 << 2) != 0 {
        wParam |= MK::MBUTTON;
    }

    MSG {
        hwnd,
        message: wm as u32,
        wParam: wParam.bits(),
        lParam: (message.y as u16 as u32) << 16 | message.x as u16 as u32,
        time: 0, // todo
        // TODO: screen coordinates
        pt: POINT {
            x: message.x as i32,
            y: message.y as i32,
        },
    }
}

impl MessageQueue {
    fn paint_msg(&self) -> Option<MSG> {
        let window = self.window.as_ref()?.borrow();
        if !window.dirty {
            return None;
        }

        Some(MSG {
            hwnd: window.hwnd,
            message: WM::PAINT as u32,
            wParam: 0,
            lParam: 0,
            time: 0,
            pt: POINT::default(),
        })
    }

    fn peek(&mut self) -> Option<MSG> {
        if let Some(msg) = self.messages.front() {
            Some(*msg)
        } else if self.quit.is_some() {
            self.quit
        } else {
            self.paint_msg()
        }
    }

    fn pop(&mut self) -> Option<MSG> {
        if let Some(msg) = self.messages.pop_front() {
            Some(msg)
        } else if self.quit.is_some() {
            self.quit.take()
        } else {
            self.paint_msg()
        }
    }

    /// Pop one message, waiting for a new one if necessary.
    fn read(&mut self) -> MSG {
        loop {
            if let Some(msg) = self.pop() {
                return msg;
            }
            self.wait_host();
        }
    }

    /// Read one pending host message, if any available.
    fn poll_host(&mut self) {
        let Some(message) = host::host().poll() else {
            return;
        };
        self.enqueue_message(message);
    }

    /// Wait for a new message to arrive.
    fn wait_host(&mut self) {
        let message = host::host().wait();
        self.enqueue_message(message);
    }

    fn enqueue_message(&mut self, msg: host::Message) {
        if matches!(msg, host::Message::Paint) {
            if let Some(window) = &self.window {
                window.borrow_mut().dirty = true;
            }
            return;
        }

        let msg = self.msg_from_message(msg);
        if *LOG_MESSAGES {
            log::info!("{:#x?}", msg);
        }

        // PAINT/TIMER/QUIT are in special queues.
        if msg.message == WM::QUIT as u32 {
            self.quit = Some(msg);
        } else {
            self.messages.push_back(msg);
        }
    }

    fn msg_from_message(&self, message: host::Message) -> MSG {
        use host::Message::*;
        let hwnd = self.window.as_ref().unwrap().borrow().hwnd;
        match message {
            Paint => unreachable!(),
            MouseDown(mouse) => mouse_msg(mouse_button_to_wm(true, &mouse), hwnd, &mouse),
            MouseUp(mouse) => mouse_msg(mouse_button_to_wm(false, &mouse), hwnd, &mouse),
            MouseMove(mouse) => mouse_msg(WM::MOUSEMOVE, hwnd, &mouse),
            Quit => {
                MSG {
                    hwnd,
                    message: WM::QUIT as u32,
                    wParam: 0, // todo
                    lParam: 0, // todo
                    time: 0,   // todo
                    pt: POINT::default(),
                }
            }
        }
    }
}

#[win32_derive::dllexport]
pub fn DispatchMessageA(ctx: &mut Context, lpMsg: Ptr<MSG>) -> u32 {
    DispatchMessageW(ctx, lpMsg)
}

#[win32_derive::dllexport]
pub fn DispatchMessageW(ctx: &mut Context, lpMsg: Ptr<MSG>) -> u32 {
    let wndproc = state().wndclass.borrow().as_ref().unwrap().wndproc.clone();
    let msg = lpMsg.read(&ctx.memory).unwrap();
    // WNDPROC
    ctx.call_x86(
        wndproc,
        vec![msg.hwnd.to_raw(), msg.message, msg.wParam, msg.lParam],
    );
    0
}

#[win32_derive::dllexport]
pub fn TranslateMessage(_ctx: &mut Context, _lpMsg: Ptr<MSG>) -> bool {
    false // no translation
}

#[win32_derive::dllexport]
pub fn PeekMessageA(
    ctx: &mut Context,
    lpMsg: Ptr<MSG>,
    hWnd: HWND,
    _wMsgFilterMin: u32,
    _wMsgFilterMax: u32,
    wRemoveMsg: u32, /* PEEK_MESSAGE_REMOVE_TYPE */
) -> bool {
    let remove = match wRemoveMsg {
        0 => false,   // PM_NOREMOVE
        1 => true,    // PM_REMOVE
        _ => todo!(), // e.g. PM_NOYIELD
    };
    let mut queue = state().message_queue.borrow_mut();
    queue.poll_host();
    let Some(msg) = queue.peek() else {
        return false;
    };

    if hWnd.is_null() {
    } else if hWnd.is_invalid() {
        // TODO: only null hwnd messages
        assert!(msg.hwnd.is_null());
    } else {
        // TODO: only matching messages
        assert_eq!(msg.hwnd, hWnd);
    }
    lpMsg.write(&mut ctx.memory, msg).unwrap();
    if remove {
        queue.pop();
    }
    true
}

#[win32_derive::dllexport]
pub fn PeekMessageW(
    ctx: &mut Context,
    lpMsg: Ptr<MSG>,
    hWnd: HWND,
    wMsgFilterMin: u32,
    wMsgFilterMax: u32,
    wRemoveMsg: u32, /* PEEK_MESSAGE_REMOVE_TYPE */
) -> bool {
    PeekMessageA(ctx, lpMsg, hWnd, wMsgFilterMin, wMsgFilterMax, wRemoveMsg)
}

#[win32_derive::dllexport]
pub fn GetMessageA(
    ctx: &mut Context,
    lpMsg: Ptr<MSG>,
    hWnd: HWND,
    wMsgFilterMin: u32,
    wMsgFilterMax: u32,
) -> i32 {
    GetMessageW(ctx, lpMsg, hWnd, wMsgFilterMin, wMsgFilterMax)
}

#[win32_derive::dllexport]
pub fn GetMessageW(
    ctx: &mut Context,
    lpMsg: Ptr<MSG>,
    hWnd: HWND,
    _wMsgFilterMin: u32,
    _wMsgFilterMax: u32,
) -> i32 {
    let msg = state().message_queue.borrow_mut().read();
    if msg.message == WM::QUIT as u32 {
        return 0;
    }

    if hWnd.is_null() {
    } else if hWnd.is_invalid() {
        // TODO: only null hwnd messages
        assert!(msg.hwnd.is_null());
    } else {
        // TODO: only matching messages
        assert_eq!(msg.hwnd, hWnd);
    }
    lpMsg.write(&mut ctx.memory, msg).unwrap();

    1 // no error, no WM_QUIT
}

#[win32_derive::dllexport]
pub fn TranslateAcceleratorW(
    _ctx: &mut Context,
    _hWnd: HWND,
    _hAccTable: HACCEL,
    _lpMsg: Ptr<MSG>,
) -> i32 {
    stub!(0) // no translation
}

#[win32_derive::dllexport]
pub fn PostQuitMessage(_ctx: &mut Context, _nExitCode: i32) {
    todo!()
}

#[win32_derive::dllexport]
pub fn PostMessageW(
    _ctx: &mut Context,
    _hWnd: HWND,
    _Msg: u32,
    _wParam: WPARAM,
    _lParam: LPARAM,
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn SendMessageW(
    _ctx: &mut Context,
    _hWnd: HWND,
    _Msg: u32,
    _wParam: WPARAM,
    _lParam: LPARAM,
) -> u32 {
    todo!()
}
