//! DirectInput. Devices exist and can be acquired; GetDeviceState/GetDeviceData
//! report no input until the host grows keyboard support (see the retrowin32
//! fork's dinput for the full reference).

use std::{cell::OnceCell, collections::HashMap, sync::Mutex};

use runtime::Context;

use crate::{ddraw::GUID, heap::Heap, kernel32};

const GUID_SysMouse: GUID = GUID((
    0x6F1D2B60,
    0xD5A0,
    0x11CF,
    [0xBF, 0xC7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00],
));

const GUID_SysKeyboard: GUID = GUID((
    0x6F1D2B61,
    0xD5A0,
    0x11CF,
    [0xBF, 0xC7, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00],
));

const DI_OK: u32 = 0;
const DIERR_DEVICENOTREG: u32 = 0x80040154;

/// Which physical device a created IDirectInputDevice stands for.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DeviceKind {
    Keyboard,
    Mouse,
}

#[derive(Default)]
pub struct State {
    /// Maps an IDirectInputDevice interface pointer to the device it represents.
    pub devices: Mutex<HashMap<u32, DeviceKind>>,
}

struct StaticState(OnceCell<State>);
unsafe impl Sync for StaticState {}
static STATE: StaticState = StaticState(OnceCell::new());

pub fn state() -> &'static State {
    STATE.0.get_or_init(Default::default)
}

pub const VTABLES: [(&'static str, &[&str]); 2] = [
    ("IDirectInput", IDirectInput::VTABLE_ENTRIES.as_slice()),
    (
        "IDirectInputDevice",
        IDirectInputDevice::VTABLE_ENTRIES.as_slice(),
    ),
];

#[win32_derive::dllexport]
pub fn DirectInputCreateA(
    ctx: &mut Context,
    _hinst: u32,
    _dwVersion: u32,
    ppDI: u32,
    _punkOuter: u32,
) -> u32 {
    let mut kernel32 = kernel32::lock();
    let ptr = IDirectInput::new(ctx, &mut kernel32.process_heap);
    drop(kernel32);
    ctx.memory.write::<u32>(ppDI, ptr);
    DI_OK
}

pub mod IDirectInput {
    use super::*;

    pub const VTABLE_ENTRIES: [&str; 8] = [
        "QueryInterface",
        "AddRef",
        "Release",
        "CreateDevice",
        "EnumDevices",
        "GetDeviceStatus",
        "RunControlPanel",
        "Initialize",
    ];

    pub static mut VTABLE: u32 = 0;

    pub fn new(ctx: &mut Context, heap: &mut Heap) -> u32 {
        let addr = heap.alloc(&mut ctx.memory, 4);
        ctx.memory.write(addr, unsafe { VTABLE });
        addr
    }

    #[win32_derive::dllexport]
    pub fn QueryInterface(_ctx: &mut Context, _this: u32, _riid: u32, _ppv: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn AddRef(_ctx: &mut Context, _this: u32) -> u32 {
        1
    }

    #[win32_derive::dllexport]
    pub fn Release(_ctx: &mut Context, _this: u32) -> u32 {
        0
    }

    #[win32_derive::dllexport]
    pub fn CreateDevice(
        ctx: &mut Context,
        _this: u32,
        lpGUID: u32,
        lplpDirectInputDevice: u32,
        _pUnkOuter: u32,
    ) -> u32 {
        let guid = crate::Ptr::<GUID>::new(lpGUID).read(&ctx.memory).unwrap();
        let kind = if guid == GUID_SysKeyboard {
            DeviceKind::Keyboard
        } else if guid == GUID_SysMouse {
            DeviceKind::Mouse
        } else {
            log::warn!("CreateDevice: unknown GUID {guid:?}");
            return DIERR_DEVICENOTREG;
        };
        let mut kernel32 = kernel32::lock();
        let device = IDirectInputDevice::new(ctx, &mut kernel32.process_heap);
        drop(kernel32);
        state().devices.lock().unwrap().insert(device, kind);
        ctx.memory.write::<u32>(lplpDirectInputDevice, device);
        DI_OK
    }

    #[win32_derive::dllexport]
    pub fn EnumDevices(
        _ctx: &mut Context,
        _this: u32,
        _dwDevType: u32,
        _callback: u32,
        _pvRef: u32,
        _dwFlags: u32,
    ) -> u32 {
        DI_OK
    }

    #[win32_derive::dllexport]
    pub fn GetDeviceStatus(_ctx: &mut Context, _this: u32, _rguid: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn RunControlPanel(_ctx: &mut Context, _this: u32, _hwnd: u32, _dwFlags: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Initialize(_ctx: &mut Context, _this: u32, _hinst: u32, _dwVersion: u32) -> u32 {
        DI_OK
    }
}

pub mod IDirectInputDevice {
    use super::*;

    // IDirectInputDevice2 layout; the game may call Poll() before reading state.
    pub const VTABLE_ENTRIES: [&str; 27] = [
        "QueryInterface",
        "AddRef",
        "Release",
        "GetCapabilities",
        "EnumObjects",
        "GetProperty",
        "SetProperty",
        "Acquire",
        "Unacquire",
        "GetDeviceState",
        "GetDeviceData",
        "SetDataFormat",
        "SetEventNotification",
        "SetCooperativeLevel",
        "GetObjectInfo",
        "GetDeviceInfo",
        "RunControlPanel",
        "Initialize",
        "CreateEffect",
        "EnumEffects",
        "GetEffectInfo",
        "GetForceFeedbackState",
        "SendForceFeedbackCommand",
        "EnumCreatedEffectObjects",
        "Escape",
        "Poll",
        "SendDeviceData",
    ];

    pub static mut VTABLE: u32 = 0;

    pub fn new(ctx: &mut Context, heap: &mut Heap) -> u32 {
        let addr = heap.alloc(&mut ctx.memory, 4);
        ctx.memory.write(addr, unsafe { VTABLE });
        addr
    }

    pub fn device_kind(this: u32) -> DeviceKind {
        state()
            .devices
            .lock()
            .unwrap()
            .get(&this)
            .copied()
            .unwrap_or(DeviceKind::Keyboard)
    }

    #[win32_derive::dllexport]
    pub fn QueryInterface(_ctx: &mut Context, _this: u32, _riid: u32, _ppv: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn AddRef(_ctx: &mut Context, _this: u32) -> u32 {
        1
    }

    #[win32_derive::dllexport]
    pub fn Release(_ctx: &mut Context, _this: u32) -> u32 {
        state().devices.lock().unwrap().remove(&_this);
        0
    }

    #[win32_derive::dllexport]
    pub fn GetCapabilities(_ctx: &mut Context, _this: u32, _lpCaps: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn EnumObjects(
        _ctx: &mut Context,
        _this: u32,
        _lpCallback: u32,
        _pvRef: u32,
        _dwFlags: u32,
    ) -> u32 {
        DI_OK
    }

    #[win32_derive::dllexport]
    pub fn GetProperty(_ctx: &mut Context, _this: u32, _rguidProp: u32, _pdiph: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn SetProperty(_ctx: &mut Context, _this: u32, _rguidProp: u32, _pdiph: u32) -> u32 {
        DI_OK
    }

    #[win32_derive::dllexport]
    pub fn Acquire(_ctx: &mut Context, _this: u32) -> u32 {
        DI_OK
    }

    #[win32_derive::dllexport]
    pub fn Unacquire(_ctx: &mut Context, _this: u32) -> u32 {
        DI_OK
    }

    /// Keyboard: a byte array indexed by DIK scancode (0x80 = pressed).
    /// Mouse: DIMOUSESTATE (lX/lY/lZ relative, then button bytes).
    /// No host input plumbing yet, so everything reads as idle.
    #[win32_derive::dllexport]
    pub fn GetDeviceState(ctx: &mut Context, _this: u32, cbData: u32, lpvData: u32) -> u32 {
        ctx.memory[lpvData..][..cbData as usize].fill(0);
        DI_OK
    }

    #[win32_derive::dllexport]
    pub fn GetDeviceData(
        ctx: &mut Context,
        _this: u32,
        _cbObjectData: u32,
        _rgdod: u32,
        pdwInOut: u32,
        _dwFlags: u32,
    ) -> u32 {
        if pdwInOut != 0 {
            ctx.memory.write::<u32>(pdwInOut, 0); // no buffered events
        }
        DI_OK
    }

    #[win32_derive::dllexport]
    pub fn SetDataFormat(_ctx: &mut Context, _this: u32, _lpdf: u32) -> u32 {
        DI_OK
    }

    #[win32_derive::dllexport]
    pub fn SetEventNotification(_ctx: &mut Context, _this: u32, _hEvent: u32) -> u32 {
        DI_OK
    }

    #[win32_derive::dllexport]
    pub fn SetCooperativeLevel(_ctx: &mut Context, _this: u32, _hwnd: u32, _dwFlags: u32) -> u32 {
        DI_OK
    }

    #[win32_derive::dllexport]
    pub fn GetObjectInfo(
        _ctx: &mut Context,
        _this: u32,
        _pdidoi: u32,
        _dwObj: u32,
        _dwHow: u32,
    ) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetDeviceInfo(_ctx: &mut Context, _this: u32, _pdidi: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn RunControlPanel(_ctx: &mut Context, _this: u32, _hwnd: u32, _dwFlags: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Initialize(
        _ctx: &mut Context,
        _this: u32,
        _hinst: u32,
        _dwVersion: u32,
        _rguid: u32,
    ) -> u32 {
        DI_OK
    }

    #[win32_derive::dllexport]
    pub fn CreateEffect(_ctx: &mut Context, _this: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn EnumEffects(_ctx: &mut Context, _this: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetEffectInfo(_ctx: &mut Context, _this: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn GetForceFeedbackState(_ctx: &mut Context, _this: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn SendForceFeedbackCommand(_ctx: &mut Context, _this: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn EnumCreatedEffectObjects(_ctx: &mut Context, _this: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Escape(_ctx: &mut Context, _this: u32) -> u32 {
        todo!()
    }

    #[win32_derive::dllexport]
    pub fn Poll(_ctx: &mut Context, _this: u32) -> u32 {
        DI_OK
    }

    #[win32_derive::dllexport]
    pub fn SendDeviceData(_ctx: &mut Context, _this: u32) -> u32 {
        todo!()
    }
}
