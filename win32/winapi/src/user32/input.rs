//! Global input state.
//!
//! There is one source of input events — the host message pump in message.rs —
//! and several consumers: window messages (WM_KEYDOWN etc), the polling APIs
//! (GetKeyState/GetAsyncKeyState), and DirectInput. They all read the state
//! maintained here, so they can never disagree about which keys are down.

use std::collections::VecDeque;

use runtime::Context;

use crate::user32::state;

/// "Key is down" bit, both in GetKeyState's result and in a DirectInput device
/// state array.
pub const KEY_DOWN: u8 = 0x80;
/// "Key is toggled on" bit of GetKeyState's result (caps lock and friends).
const KEY_TOGGLED: u8 = 0x01;

/// DirectInput drops the oldest events when a device's buffer fills. Apps ask
/// for a size via DIPROP_BUFFERSIZE; this caps what we'll honor so an app that
/// never drains its buffer can't grow us without bound.
const MAX_BUFFER_SIZE: usize = 1024;

/// One buffered device event, in the shape IDirectInputDevice::GetDeviceData
/// reports: an offset within the device's data format plus its new value.
#[derive(Clone, Copy)]
pub struct DeviceEvent {
    /// Offset of the changed object within the device data format. For a
    /// keyboard that is the DirectInput scan code; for a mouse, the axis or
    /// button offset within DIMOUSESTATE.
    pub ofs: u32,
    pub data: u32,
    /// Host time in milliseconds.
    pub time: u32,
    /// DirectInput hands out a monotonic sequence number per event.
    pub sequence: u32,
}

/// Byte offset of a mouse object within the standard c_dfDIMouse data format:
/// lX@0, lY@4, lZ@8, then one byte per button from 12.
const MOUSE_AXIS_X: u32 = 0;
const MOUSE_AXIS_Y: u32 = 4;
const MOUSE_BUTTON_0: u32 = 12;
pub const MOUSE_BUTTONS: usize = 4;

pub struct Mouse {
    /// Cursor position in client coordinates.
    pub x: i32,
    pub y: i32,
    /// Movement accumulated since DirectInput last read it. DirectInput mice
    /// report relative motion, so reading resets this.
    dx: i32,
    dy: i32,
    /// Button state, indexed as DIMOUSESTATE::rgbButtons: left, right, middle.
    pub buttons: [u8; MOUSE_BUTTONS],
    /// False until the first host mouse event, so the initial position doesn't
    /// register as a jump.
    have_position: bool,
}

impl Default for Mouse {
    fn default() -> Self {
        Mouse {
            x: 0,
            y: 0,
            dx: 0,
            dy: 0,
            buttons: [0; MOUSE_BUTTONS],
            have_position: false,
        }
    }
}

impl Mouse {
    /// Consume the accumulated relative motion, as a read of a relative-axis
    /// device does.
    pub fn take_motion(&mut self) -> (i32, i32) {
        let motion = (self.dx, self.dy);
        self.dx = 0;
        self.dy = 0;
        motion
    }

    /// Move the tracked cursor without generating relative motion, so a
    /// SetCursorPos warp isn't mistaken for the user moving the mouse.
    pub fn warp(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
        self.have_position = true;
    }
}

/// A device's buffered event queue. Empty `buffer_size` means the app never
/// asked for buffered input, in which case DirectInput buffers nothing.
#[derive(Default)]
struct EventBuffer {
    events: VecDeque<DeviceEvent>,
    size: usize,
    overflowed: bool,
}

impl EventBuffer {
    fn push(&mut self, event: DeviceEvent) {
        if self.size == 0 {
            return;
        }
        if self.events.len() >= self.size {
            self.events.pop_front();
            self.overflowed = true;
        }
        self.events.push_back(event);
    }

    /// Take up to `max` events, oldest first. `peek` leaves them buffered, as
    /// DIGDD_PEEK asks. Returns the events plus whether an overflow happened
    /// since the last non-peeking read.
    fn take(&mut self, max: usize, peek: bool) -> (Vec<DeviceEvent>, bool) {
        let count = max.min(self.events.len());
        let overflowed = self.overflowed;
        if peek {
            return (
                self.events.iter().take(count).copied().collect(),
                overflowed,
            );
        }
        self.overflowed = false;
        (self.events.drain(..count).collect(), overflowed)
    }
}

pub struct Input {
    /// Indexed by VK_* code, holding GetKeyState's KEY_DOWN|KEY_TOGGLED bits.
    keys: [u8; 256],
    /// Indexed by DirectInput scan code, holding KEY_DOWN.
    dik: [u8; 256],
    /// Keys pressed since GetAsyncKeyState last reported them, by VK_* code.
    async_pressed: [bool; 256],
    keyboard_buffer: EventBuffer,
    mouse_buffer: EventBuffer,
    next_sequence: u32,
    pub mouse: Mouse,
}

impl Default for Input {
    fn default() -> Self {
        Input {
            keys: [0; 256],
            dik: [0; 256],
            async_pressed: [false; 256],
            keyboard_buffer: Default::default(),
            mouse_buffer: Default::default(),
            next_sequence: 0,
            mouse: Default::default(),
        }
    }
}

/// Maps a side-specific virtual key (VK_LSHIFT) to the generic one (VK_SHIFT),
/// and to the opposite side's key. Window messages carry the generic code,
/// while the key state array holds both.
fn key_sides(vkey: u8) -> Option<(u8, u8)> {
    Some(match vkey {
        0xa0 => (0x10, 0xa1), // VK_LSHIFT -> VK_SHIFT, VK_RSHIFT
        0xa1 => (0x10, 0xa0),
        0xa2 => (0x11, 0xa3), // VK_LCONTROL -> VK_CONTROL, VK_RCONTROL
        0xa3 => (0x11, 0xa2),
        0xa4 => (0x12, 0xa5), // VK_LMENU -> VK_MENU, VK_RMENU
        0xa5 => (0x12, 0xa4),
        _ => return None,
    })
}

/// The virtual key a window message reports for this key: the generic code
/// where there is one.
pub fn message_vkey(vkey: u8) -> u8 {
    key_sides(vkey).map_or(vkey, |(generic, _)| generic)
}

/// Keys that toggle rather than just repeat: caps lock, num lock, scroll lock.
fn is_toggle_key(vkey: u8) -> bool {
    matches!(vkey, 0x14 | 0x90 | 0x91)
}

impl Input {
    pub fn key_state(&self, vkey: u8) -> u8 {
        self.keys[vkey as usize]
    }

    pub fn key_down(&self, vkey: u8) -> bool {
        self.keys[vkey as usize] & KEY_DOWN != 0
    }

    /// The 256-byte array a DirectInput keyboard reports.
    pub fn dik_state(&self) -> &[u8; 256] {
        &self.dik
    }

    fn next_sequence(&mut self) -> u32 {
        let sequence = self.next_sequence;
        self.next_sequence = self.next_sequence.wrapping_add(1);
        sequence
    }

    fn set_key(&mut self, vkey: u8, down: bool, toggle: bool) {
        let state = &mut self.keys[vkey as usize];
        if down {
            if toggle && is_toggle_key(vkey) {
                *state ^= KEY_TOGGLED;
            }
            *state |= KEY_DOWN;
            self.async_pressed[vkey as usize] = true;
        } else {
            *state &= !KEY_DOWN;
        }
    }

    /// Whether the key was pressed since this was last asked, clearing the
    /// flag; the low bit of GetAsyncKeyState's result.
    pub fn take_async_press(&mut self, vkey: u8) -> bool {
        std::mem::take(&mut self.async_pressed[vkey as usize])
    }

    pub fn on_key(&mut self, key: &host::KeyMessage, down: bool) {
        self.set_key(key.vkey, down, !key.repeat);
        if let Some((generic, other)) = key_sides(key.vkey) {
            // The generic entry stays down while either side is held.
            if down || !self.key_down(other) {
                self.set_key(generic, down, false);
            }
        }

        let dik = key.scancode | if key.extended { 0x80 } else { 0 };
        // DirectInput reports edges, not auto-repeat.
        if key.repeat {
            return;
        }
        self.dik[dik as usize] = if down { KEY_DOWN } else { 0 };
        let sequence = self.next_sequence();
        self.keyboard_buffer.push(DeviceEvent {
            ofs: dik as u32,
            data: if down { KEY_DOWN as u32 } else { 0 },
            time: host::host().time(),
            sequence,
        });
    }

    pub fn on_mouse(&mut self, mouse: &host::MouseMessage) {
        let (x, y) = (mouse.x as i32, mouse.y as i32);
        if self.mouse.have_position {
            let (dx, dy) = (x - self.mouse.x, y - self.mouse.y);
            for (ofs, delta) in [(MOUSE_AXIS_X, dx), (MOUSE_AXIS_Y, dy)] {
                if delta == 0 {
                    continue;
                }
                let sequence = self.next_sequence();
                self.mouse_buffer.push(DeviceEvent {
                    ofs,
                    data: delta as u32,
                    time: host::host().time(),
                    sequence,
                });
            }
            self.mouse.dx += dx;
            self.mouse.dy += dy;
        }
        self.mouse.have_position = true;
        self.mouse.x = x;
        self.mouse.y = y;

        // DIMOUSESTATE orders buttons left, right, middle.
        let buttons = [
            host::MouseButton::Left,
            host::MouseButton::Right,
            host::MouseButton::Middle,
        ];
        for (index, button) in buttons.into_iter().enumerate() {
            let down = if mouse.buttons.contains(button) {
                KEY_DOWN
            } else {
                0
            };
            if self.mouse.buttons[index] == down {
                continue;
            }
            self.mouse.buttons[index] = down;
            let sequence = self.next_sequence();
            self.mouse_buffer.push(DeviceEvent {
                ofs: MOUSE_BUTTON_0 + index as u32,
                data: down as u32,
                time: host::host().time(),
                sequence,
            });
        }
    }

    pub fn buffer_size(&self, keyboard: bool) -> usize {
        if keyboard {
            self.keyboard_buffer.size
        } else {
            self.mouse_buffer.size
        }
    }

    pub fn set_buffer_size(&mut self, keyboard: bool, size: usize) {
        let buffer = if keyboard {
            &mut self.keyboard_buffer
        } else {
            &mut self.mouse_buffer
        };
        buffer.size = size.min(MAX_BUFFER_SIZE);
        buffer.events.clear();
    }

    pub fn take_events(
        &mut self,
        keyboard: bool,
        max: usize,
        peek: bool,
    ) -> (Vec<DeviceEvent>, bool) {
        let buffer = if keyboard {
            &mut self.keyboard_buffer
        } else {
            &mut self.mouse_buffer
        };
        buffer.take(max, peek)
    }
}

/// Drain every host event that is ready, updating input state.
///
/// Window message dispatch does this as a side effect of pumping messages, but
/// DirectInput reads state without touching the message queue, so it calls
/// this first to avoid reporting a stale keyboard.
pub fn pump_host_input() {
    state().message_queue.borrow_mut().poll_host_all();
}

fn vkey_to_char(vkey: u8, shift: bool, caps: bool) -> Option<u8> {
    Some(match vkey {
        // Letters are upper case unless exactly one of shift/caps is on.
        b'A'..=b'Z' => {
            if shift != caps {
                vkey
            } else {
                vkey.to_ascii_lowercase()
            }
        }
        b'0'..=b'9' => {
            if shift {
                b")!@#$%^&*("[(vkey - b'0') as usize]
            } else {
                vkey
            }
        }
        0x08 | 0x09 | 0x1b | 0x20 => vkey, // backspace, tab, escape, space
        0x0d => b'\r',
        0x60..=0x69 => b'0' + (vkey - 0x60), // numpad digits
        0x6a => b'*',
        0x6b => b'+',
        0x6d => b'-',
        0x6e => b'.',
        0x6f => b'/',
        0xba => shifted(shift, b';', b':'),
        0xbb => shifted(shift, b'=', b'+'),
        0xbc => shifted(shift, b',', b'<'),
        0xbd => shifted(shift, b'-', b'_'),
        0xbe => shifted(shift, b'.', b'>'),
        0xbf => shifted(shift, b'/', b'?'),
        0xc0 => shifted(shift, b'`', b'~'),
        0xdb => shifted(shift, b'[', b'{'),
        0xdc => shifted(shift, b'\\', b'|'),
        0xdd => shifted(shift, b']', b'}'),
        0xde => shifted(shift, b'\'', b'"'),
        _ => return None,
    })
}

fn shifted(shift: bool, plain: u8, shifted: u8) -> u8 {
    if shift { shifted } else { plain }
}

/// The character a WM_KEYDOWN translates to, or None for keys that produce no
/// text. Used by TranslateMessage.
pub fn char_for_key(vkey: u8) -> Option<u8> {
    let input = state().input.borrow();
    let shift = input.key_down(0x10);
    let caps = input.key_state(0x14) & KEY_TOGGLED != 0;
    vkey_to_char(vkey, shift, caps)
}

#[win32_derive::dllexport]
pub fn GetKeyState(_ctx: &mut Context, nVirtKey: u32) -> i16 {
    let state = state().input.borrow().key_state(nVirtKey as u8);
    ((state as i16) << 8) | (state & KEY_TOGGLED) as i16
}

#[win32_derive::dllexport]
pub fn GetAsyncKeyState(_ctx: &mut Context, vKey: u32) -> i16 {
    pump_host_input();
    // The low bit means "pressed since the last call for this key", which is
    // how games poll for a fresh press, so it has to be consumed here.
    let pressed_since = state().input.borrow_mut().take_async_press(vKey as u8);
    let state = state().input.borrow().key_state(vKey as u8);
    ((state as i16) << 8) | (pressed_since as i16)
}

#[win32_derive::dllexport]
pub fn GetKeyboardState(ctx: &mut Context, lpKeyState: u32) -> bool {
    let input = state().input.borrow();
    for vkey in 0..256u32 {
        ctx.memory[lpKeyState + vkey] = input.key_state(vkey as u8);
    }
    true
}
