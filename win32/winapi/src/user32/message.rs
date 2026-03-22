use std::collections::VecDeque;

use crate::{
    stub,
    user32::{HWND, state},
};

struct MSG {}

#[derive(Default)]
pub struct MessageQueue {
    messages: VecDeque<MSG>,
}

impl MessageQueue {
    fn peek(&mut self) -> Option<&MSG> {
        if self.messages.is_empty() {
            self.pump();
        }
        self.messages.front()
    }

    fn pump(&mut self) {
        let ev = state().event_pump.borrow_mut().poll_event();
        dbg!(ev);
    }
}

#[win32_derive::dllexport]
pub fn DispatchMessageA(_lpMsg: u32) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn TranslateMessage(_lpMsg: u32) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn PeekMessageA(
    _lpMsg: u32,
    _hWnd: HWND,
    _wMsgFilterMin: u32,
    _wMsgFilterMax: u32,
    _wRemoveMsg: u32, /* PEEK_MESSAGE_REMOVE_TYPE */
) -> bool {
    state().message_queue.borrow_mut().peek();
    stub!(false)
}

#[win32_derive::dllexport]
pub fn GetMessageA(_lpMsg: u32, _hWnd: HWND, _wMsgFilterMin: u32, _wMsgFilterMax: u32) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn PostQuitMessage(_nExitCode: i32) {
    todo!()
}
