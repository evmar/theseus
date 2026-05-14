use std::collections::VecDeque;
use zerocopy::{FromBytes, IntoBytes};

use runtime::Context;

use crate::{
    POINT, stub,
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

impl MessageQueue {
    fn peek(&mut self) -> Option<&MSG> {
        if self.messages.is_empty() {
            self.poll();
        }
        self.messages.front()
    }

    fn get(&mut self) -> MSG {
        loop {
            if let Some(msg) = self.messages.pop_front() {
                return msg;
            }
            self.wait();
        }
    }

    fn poll(&mut self) {
        let Some(event) = state().event_pump.borrow_mut().poll_event() else {
            return;
        };
        let Some(msg) = self.msg_from_event(event) else {
            return;
        };
        self.messages.push_back(msg);
    }

    fn wait(&mut self) {
        let event = state().event_pump.borrow_mut().wait_event();
        let Some(msg) = self.msg_from_event(event) else {
            return;
        };
        self.messages.push_back(msg);
    }

    fn msg_from_event(&self, event: sdl3::event::Event) -> Option<MSG> {
        use sdl3::event::Event;
        match event {
            Event::Window { win_event, .. } => {
                use sdl3::event::WindowEvent;
                match win_event {
                    WindowEvent::Shown => return None,
                    WindowEvent::Resized(_, _) => return None,
                    WindowEvent::FocusGained => return None,
                    WindowEvent::FocusLost => return None,
                    WindowEvent::Exposed => {
                        return Some(MSG {
                            hwnd: self.hwnd,
                            message: 0xf, // WM_PAINT,
                            wParam: 0,    // todo
                            lParam: 0,    // todo
                            time: 0,      // todo
                            pt: POINT::default(),
                        });
                    }
                    WindowEvent::MouseEnter => return None,
                    WindowEvent::MouseLeave => return None,
                    WindowEvent::PixelSizeChanged(_, _) => return None,
                    _ => {}
                }
            }
            Event::MouseMotion { .. } => {
                return None;
            }
            Event::AudioDeviceAdded { .. }
            | Event::ClipboardUpdate { .. }
            | Event::Unknown { .. } => {
                // ignore
                return None;
            }
            _ => {}
        }
        log::warn!("todo: handle sdl event: {:?}", event);
        None
    }
}

#[win32_derive::dllexport]
pub fn DispatchMessageA(_ctx: &mut Context, _lpMsg: u32) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn DispatchMessageW(ctx: &mut Context, lpMsg: u32 /* MSG */) -> u32 {
    let wndproc = state().wnd_class.borrow().as_ref().unwrap().wndproc.clone();
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
    _ctx: &mut Context,
    _lpMsg: u32,
    _hWnd: HWND,
    _wMsgFilterMin: u32,
    _wMsgFilterMax: u32,
    _wRemoveMsg: u32, /* PEEK_MESSAGE_REMOVE_TYPE */
) -> bool {
    let mut queue = state().message_queue.borrow_mut();
    let front = queue.peek();
    assert!(front.is_none()); // TODO
    false
}

#[win32_derive::dllexport]
pub fn GetMessageA(
    _ctx: &mut Context,
    _lpMsg: u32,
    _hWnd: HWND,
    _wMsgFilterMin: u32,
    _wMsgFilterMax: u32,
) -> i32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetMessageW(
    ctx: &mut Context,
    lpMsg: u32, /* MSG */
    hWnd: HWND,
    _wMsgFilterMin: u32,
    _wMsgFilterMax: u32,
) -> i32 {
    let msg = state().message_queue.borrow_mut().get();

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
