use std::{collections::VecDeque, sync::LazyLock};

use runtime::Context;

use crate::{
    POINT, Ptr,
    dllexport::win32flags,
    host, stub, trace,
    user32::{HACCEL, HWND, state},
};

/// If THESEUS_TRACE includes "wm", log all Windows messages.
static LOG_MESSAGES: LazyLock<bool> =
    LazyLock::new(|| !matches!(trace::get_uncached("wm"), trace::Trace::None));

pub type WPARAM = u32;
pub type LPARAM = u32;

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
    pub hwnd: HWND,
    messages: VecDeque<MSG>,
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

fn mouse_msg(is_down: bool, hwnd: HWND, message: host::MouseMessage) -> MSG {
    let wParam = match message.button {
        1 => MK::LBUTTON,
        2 => MK::RBUTTON,
        3 => MK::MBUTTON,
        _ => MK::empty(),
    }
    .bits();
    let wm = match (message.button, is_down) {
        (1, true) => 0x201,  // WM_LBUTTONDOWN
        (1, false) => 0x202, // WM_LBUTTONUP
        (2, true) => 0x204,  // WM_RBUTTONDOWN
        (2, false) => 0x205, // WM_RBUTTONUP
        (3, true) => 0x207,  // WM_MBUTTONDOWN
        (3, false) => 0x208, // WM_MBUTTONUP
        _ => unreachable!("unsupported mouse button {}", message.button),
    };
    MSG {
        hwnd,
        message: wm,
        wParam,
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
    fn peek(&mut self) -> Option<&MSG> {
        self.messages.front()
    }
    fn pop(&mut self) {
        self.messages.pop_front();
    }

    /// Pop one message, waiting for a new one if necessary.
    fn read(&mut self) -> MSG {
        loop {
            if let Some(msg) = self.messages.pop_front() {
                return msg;
            }
            self.wait();
        }
    }

    /// Read one pending host message, if any available.
    fn poll(&mut self) {
        let Some(message) = host::host().main_thread.get().poll() else {
            return;
        };
        let msg = self.msg_from_message(message);
        if *LOG_MESSAGES {
            log::info!("{:#x?}", msg);
        }
        self.messages.push_back(msg);
    }

    /// Wait for a new message to arrive.
    fn wait(&mut self) {
        let message = host::host().main_thread.get().wait();
        let msg = self.msg_from_message(message);
        if *LOG_MESSAGES {
            log::info!("{:#x?}", msg);
        }
        self.messages.push_back(msg);
    }

    fn msg_from_message(&self, message: host::Message) -> MSG {
        use host::Message::*;
        match message {
            Paint => {
                MSG {
                    hwnd: self.hwnd,
                    message: 0xf, // WM_PAINT,
                    wParam: 0,    // todo
                    lParam: 0,    // todo
                    time: 0,      // todo
                    pt: POINT::default(),
                }
            }
            MouseDown(mouse) => mouse_msg(true, self.hwnd, mouse),
            MouseUp(mouse) => mouse_msg(false, self.hwnd, mouse),
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
    queue.poll();
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
    lpMsg.write(&mut ctx.memory, *msg).unwrap();
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
