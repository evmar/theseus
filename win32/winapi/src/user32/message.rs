use std::collections::VecDeque;
use zerocopy::{FromBytes, IntoBytes};

use runtime::Context;

use crate::{
    POINT,
    dllexport::win32flags,
    host, stub,
    user32::{HACCEL, HWND, state},
};

pub type WPARAM = u32;
pub type LPARAM = u32;

#[repr(C)]
#[derive(zerocopy::FromBytes, zerocopy::IntoBytes, zerocopy::Immutable)]
struct MSG {
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

fn mouse_msg(hwnd: HWND, wm: u32, message: host::MouseMessage) -> MSG {
    let wParam = match message.button {
        1 => MK::LBUTTON,
        2 => MK::RBUTTON,
        3 => MK::MBUTTON,
        _ => MK::empty(),
    }
    .bits();
    MSG {
        hwnd,
        message: wm,
        wParam,
        lParam: (message.y as u16 as u32) << 16 | message.x as u16 as u32,
        time: 0, // todo
        pt: POINT::default(),
    }
}

impl MessageQueue {
    fn pop(&mut self) -> Option<MSG> {
        self.messages.pop_front()
    }

    fn read(&mut self) -> MSG {
        loop {
            if let Some(msg) = self.messages.pop_front() {
                return msg;
            }
            self.wait();
        }
    }

    fn poll(&mut self) {
        let Some(message) = host::host().main_thread.get().poll() else {
            return;
        };
        let msg = self.msg_from_message(message);
        self.messages.push_back(msg);
    }

    fn wait(&mut self) {
        let message = host::host().main_thread.get().wait();
        let msg = self.msg_from_message(message);
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
            MouseDown(mouse) => mouse_msg(self.hwnd, 0x201 /* WM_LBUTTONDOWN */, mouse),
            MouseUp(mouse) => mouse_msg(self.hwnd, 0x202 /* WM_LBUTTONUP */, mouse),
        }
    }
}

#[win32_derive::dllexport]
pub fn DispatchMessageA(ctx: &mut Context, lpMsg: u32) -> u32 {
    DispatchMessageW(ctx, lpMsg)
}

#[win32_derive::dllexport]
pub fn DispatchMessageW(ctx: &mut Context, lpMsg: u32 /* MSG */) -> u32 {
    let wndproc = state().wndclass.borrow().as_ref().unwrap().wndproc.clone();
    let msg = <MSG>::read_from_prefix(&ctx.memory[lpMsg..]).unwrap().0;
    // WNDPROC
    ctx.call_x86(
        wndproc,
        vec![msg.hwnd.to_raw(), msg.message, msg.wParam, msg.lParam],
    );
    0
}

#[win32_derive::dllexport]
pub fn TranslateMessage(_ctx: &mut Context, _lpMsg: u32) -> bool {
    false // no translation
}

#[win32_derive::dllexport]
pub fn PeekMessageA(
    ctx: &mut Context,
    lpMsg: u32, /* MSG */
    hWnd: HWND,
    _wMsgFilterMin: u32,
    _wMsgFilterMax: u32,
    _wRemoveMsg: u32, /* PEEK_MESSAGE_REMOVE_TYPE */
) -> bool {
    let mut queue = state().message_queue.borrow_mut();
    queue.poll();
    if let Some(msg) = queue.pop() {
        if hWnd.is_null() {
            msg.write_to_prefix(&mut ctx.memory[lpMsg..]).unwrap();
        } else if hWnd.is_invalid() {
            // TODO: only null hwnd messages
            assert!(msg.hwnd.is_null());
        } else {
            // TODO: only matching messages
            assert_eq!(msg.hwnd, hWnd);
        }
        true
    } else {
        false
    }
}

#[win32_derive::dllexport]
pub fn GetMessageA(
    ctx: &mut Context,
    lpMsg: u32,
    hWnd: HWND,
    wMsgFilterMin: u32,
    wMsgFilterMax: u32,
) -> i32 {
    GetMessageW(ctx, lpMsg, hWnd, wMsgFilterMin, wMsgFilterMax)
}

#[win32_derive::dllexport]
pub fn GetMessageW(
    ctx: &mut Context,
    lpMsg: u32, /* MSG */
    hWnd: HWND,
    _wMsgFilterMin: u32,
    _wMsgFilterMax: u32,
) -> i32 {
    let msg = state().message_queue.borrow_mut().read();

    if hWnd.is_null() {
        msg.write_to_prefix(&mut ctx.memory[lpMsg..]).unwrap();
    } else if hWnd.is_invalid() {
        // TODO: only null hwnd messages
        assert!(msg.hwnd.is_null());
    } else {
        // TODO: only matching messages
        assert_eq!(msg.hwnd, hWnd);
    }

    1 // no error, no WM_QUIT
}

#[win32_derive::dllexport]
pub fn TranslateAcceleratorW(
    _ctx: &mut Context,
    _hWnd: HWND,
    _hAccTable: HACCEL,
    _lpMsg: u32, /* MSG */
) -> i32 {
    stub!(0) // no translation
}

#[win32_derive::dllexport]
pub fn PostQuitMessage(_ctx: &mut Context, _nExitCode: i32) {
    todo!()
}
