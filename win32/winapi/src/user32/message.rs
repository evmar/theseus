use std::{cell::RefCell, collections::VecDeque, rc::Rc, sync::LazyLock};

use runtime::Context;

use crate::{
    POINT, Ptr,
    dllexport::win32flags,
    stub, trace,
    user32::{HACCEL, HWND, Window, char_for_key, message_vkey, state},
};

/// If THESEUS_TRACE includes "wm", log all Windows messages.
static LOG_MESSAGES: LazyLock<bool> =
    LazyLock::new(|| !matches!(trace::get_uncached("wm"), trace::Trace::None));

pub type WPARAM = u32;
pub type LPARAM = u32;

#[derive(win32_derive::ABIEnum, Debug)]
pub enum WM {
    ACTIVATE = 0x6,
    SETFOCUS = 0x7,
    PAINT = 0xf,
    QUIT = 0x12,
    SHOWWINDOW = 0x18,
    ACTIVATEAPP = 0x1c,
    KEYDOWN = 0x100,
    KEYUP = 0x101,
    CHAR = 0x102,
    SYSKEYDOWN = 0x104,
    SYSKEYUP = 0x105,
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
    // Can't use a match here because MouseButton is a bitfield, not an enum.
    if message.button == host::MouseButton::Left {
        if is_down {
            return WM::LBUTTONDOWN;
        } else {
            return WM::LBUTTONUP;
        }
    } else if message.button == host::MouseButton::Right {
        if is_down {
            return WM::RBUTTONDOWN;
        } else {
            return WM::RBUTTONUP;
        }
    } else if message.button == host::MouseButton::Middle {
        if is_down {
            return WM::MBUTTONDOWN;
        } else {
            return WM::MBUTTONUP;
        }
    } else {
        return WM::MOUSEMOVE;
    }
}

fn mouse_msg(wm: WM, hwnd: HWND, message: &host::MouseMessage) -> MSG {
    let mut wParam = MK::empty();
    if message.buttons.contains(host::MouseButton::Left) {
        wParam |= MK::LBUTTON;
    }
    if message.buttons.contains(host::MouseButton::Middle) {
        wParam |= MK::MBUTTON;
    }
    if message.buttons.contains(host::MouseButton::Right) {
        wParam |= MK::RBUTTON;
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

fn key_msg(hwnd: HWND, key: &host::KeyMessage, down: bool) -> MSG {
    // lParam packs the key's physical details, as documented for WM_KEYDOWN.
    let mut lParam = 1; // repeat count; the host reports repeats one at a time
    lParam |= (key.scancode as u32) << 16;
    if key.extended {
        lParam |= 1 << 24;
    }
    // Bit 29 is set while alt is held, bit 30 holds the previous key state,
    // bit 31 marks the release.
    let alt = state().input.borrow().key_down(0x12); // VK_MENU
    if alt {
        lParam |= 1 << 29;
    }
    if key.repeat || !down {
        lParam |= 1 << 30;
    }
    if !down {
        lParam |= 1 << 31;
    }

    // Keys pressed with alt held are "system" keys, as is alt itself.
    let system = alt || key.vkey == 0xa4 || key.vkey == 0xa5;
    let message = match (down, system) {
        (true, false) => WM::KEYDOWN,
        (false, false) => WM::KEYUP,
        (true, true) => WM::SYSKEYDOWN,
        (false, true) => WM::SYSKEYUP,
    };

    MSG {
        hwnd,
        message: message as u32,
        wParam: message_vkey(key.vkey) as u32,
        lParam,
        time: host::host().time(),
        pt: POINT::default(),
    }
}

/// Post a message to the application's queue (e.g. synthetic activation
/// messages from ShowWindow).
pub fn post_message(hwnd: HWND, message: u32, wParam: WPARAM, lParam: LPARAM) {
    let mut queue = state().message_queue.borrow_mut();
    queue.messages.push_back(MSG {
        hwnd,
        message,
        wParam,
        lParam,
        time: 0,
        pt: POINT::default(),
    });
}

#[win32_derive::dllexport]
pub fn WaitMessage(_ctx: &mut Context) -> bool {
    let mut queue = state().message_queue.borrow_mut();
    if queue.peek().is_none() {
        queue.wait_host();
    }
    true
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

    /// Read every pending host message. DirectInput calls this to refresh
    /// input state without going through the window message queue.
    pub fn poll_host_all(&mut self) {
        while let Some(message) = host::host().poll() {
            self.enqueue_message(message);
        }
    }

    /// Wait for a new message to arrive.
    fn wait_host(&mut self) {
        let message = host::host().wait();
        self.enqueue_message(message);
    }

    fn enqueue_message(&mut self, msg: host::Message) {
        #[cfg(not(target_family = "wasm"))]
        if matches!(msg, host::Message::Paint) {
            if let Some(window) = &self.window {
                window.borrow_mut().dirty = true;
            }
            return;
        }

        // Every host input event updates the shared input state, whether or not
        // the app reads it through the message queue: DirectInput reads the
        // same state, and this is the only place host events are consumed.
        {
            let mut input = state().input.borrow_mut();
            match &msg {
                host::Message::KeyDown(key) => input.on_key(key, true),
                host::Message::KeyUp(key) => input.on_key(key, false),
                host::Message::MouseDown(mouse)
                | host::Message::MouseUp(mouse)
                | host::Message::MouseMove(mouse) => input.on_mouse(mouse),
                _ => {}
            }
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
            MouseDown(mouse) => mouse_msg(mouse_button_to_wm(true, &mouse), hwnd, &mouse),
            MouseUp(mouse) => mouse_msg(mouse_button_to_wm(false, &mouse), hwnd, &mouse),
            MouseMove(mouse) => mouse_msg(WM::MOUSEMOVE, hwnd, &mouse),
            KeyDown(key) => key_msg(hwnd, &key, true),
            KeyUp(key) => key_msg(hwnd, &key, false),
            #[cfg(not(target_family = "wasm"))]
            Paint => unreachable!(),
            #[cfg(not(target_family = "wasm"))]
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
    ctx.call32_x86(
        wndproc,
        vec![msg.hwnd.to_raw(), msg.message, msg.wParam, msg.lParam],
    );
    0
}

#[win32_derive::dllexport]
pub fn TranslateMessage(ctx: &mut Context, lpMsg: Ptr<MSG>) -> bool {
    let Some(msg) = lpMsg.read(&ctx.memory) else {
        return false;
    };
    if msg.message != WM::KEYDOWN as u32 {
        return false;
    }
    let Some(ch) = char_for_key(msg.wParam as u8) else {
        return false;
    };
    // The character message follows the key message in the queue, so the app
    // sees it on its next pump.
    post_message(msg.hwnd, WM::CHAR as u32, ch as u32, msg.lParam);
    true
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
    // Games poll for messages every frame; keep the audio mixer fed from here
    // too, in case the app renders without flipping.
    crate::dsound::pump(ctx);

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
pub fn PostQuitMessage(_ctx: &mut Context, nExitCode: i32) {
    let mut queue = state().message_queue.borrow_mut();
    queue.quit = Some(MSG {
        hwnd: HWND::null(),
        message: WM::QUIT as u32,
        wParam: nExitCode as u32,
        lParam: 0,
        time: 0,
        pt: POINT::default(),
    });
}

#[win32_derive::dllexport]
pub fn PostMessageW(
    _ctx: &mut Context,
    hWnd: HWND,
    Msg: u32,
    wParam: WPARAM,
    lParam: LPARAM,
) -> bool {
    post_message(hWnd, Msg, wParam, lParam);
    true
}

#[win32_derive::dllexport]
pub fn PostMessageA(
    _ctx: &mut Context,
    hWnd: HWND,
    Msg: u32,
    wParam: WPARAM,
    lParam: LPARAM,
) -> bool {
    post_message(hWnd, Msg, wParam, lParam);
    true
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
