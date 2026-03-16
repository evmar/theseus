use runtime::*;
use std::collections::VecDeque;
use winit::platform::pump_events::EventLoopExtPumpEvents as _;

use crate::{
    stub,
    user32::{HWND, state},
};

struct H {}

impl winit::application::ApplicationHandler for H {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        super::create_pending_windows(event_loop);
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        _event: winit::event::WindowEvent,
    ) {
        //dbg!(event);
    }
}

struct MSG {}

#[derive(Default)]
pub struct MessageQueue {
    messages: VecDeque<MSG>,
}

impl MessageQueue {
    fn pump(&mut self) {
        let status = state()
            .event_loop
            .borrow_mut()
            .pump_app_events(None, &mut H {});
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
    state().message_queue.borrow_mut().pump();
    stub!(false)
}

#[win32_derive::dllexport]
pub fn PostQuitMessage(_nExitCode: i32) {
    todo!()
}
