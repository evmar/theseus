use runtime::Context;

use super::*;
use crate::{Ptr, RECT, stub};

#[win32_derive::dllexport]
pub fn GetSystemMetrics(_ctx: &mut Context, nIndex: u32 /* SYSTEM_METRICS_INDEX */) -> i32 {
    // These were dumped from a win2k VM running at 640x480.
    // See retrowin32's exe/cpp/metrics.cc.
    const METRICS: [i32; 100] = [
        640, 480, 16, 16, 19, 1, 1, 3, 3, 16, 16, 32, 32, 32, 32, 19, 640, 433, 0, 1, 16, 16, 0, 0,
        0, 0, 0, 0, 112, 27, 18, 18, 4, 4, 112, 27, 4, 4, 75, 75, 0, 0, 0, 5, 0, 2, 2, 160, 24, 16,
        16, 16, 12, 15, 18, 18, 8, 160, 24, 652, 492, 648, 460, 3, 0, 0, 0, 0, 4, 4, 0, 13, 13, 0,
        0, 1, 0, 0, 640, 480, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    METRICS[nIndex as usize]
}

#[win32_derive::dllexport]
pub fn ShowCursor(_ctx: &mut Context, bShow: bool) -> i32 {
    if bShow { stub!(1) } else { stub!(0) }
}

#[win32_derive::dllexport]
pub fn CreateCursor(
    _ctx: &mut Context,
    _hInst: HINSTANCE,
    _xHotSpot: i32,
    _yHotSpot: i32,
    _nWidth: i32,
    _nHeight: i32,
    _pvANDPlane: Ptr<u8>,
    _pvXORPlane: Ptr<u8>,
) -> HCURSOR {
    stub!(0)
}

#[win32_derive::dllexport]
pub fn ReleaseCapture(_ctx: &mut Context) -> bool {
    stub!(true)
}

#[win32_derive::dllexport]
pub fn SetCapture(_ctx: &mut Context, _hWnd: HWND) -> HWND {
    stub!(HWND::null())
}

#[win32_derive::dllexport]
pub fn WinHelpW(
    _ctx: &mut Context,
    _hWndMain: HWND,
    _lpszHelp: Ptr<u16>, /* WSTR */
    _uCommand: u32,
    _dwData: u32,
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn CheckMenuItem(_ctx: &mut Context, _hMenu: HMENU, _uIDCheckItem: u32, _uCheck: u32) -> u32 {
    stub!(0) // previously unchecked
}

pub type LRESULT = i32;

#[win32_derive::dllexport]
pub fn GetMenuItemRect(
    _ctx: &mut Context,
    _hWnd: HWND,
    _hMenu: HMENU,
    _uItem: u32,
    _lprcItem: Ptr<RECT>,
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn KillTimer(_ctx: &mut Context, _hWnd: HWND, _uIDEvent: u32) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn MessageBoxA(
    _ctx: &mut Context,
    _hWnd: HWND,
    _lpText: Ptr<u8>,
    _lpCaption: Ptr<u8>,
    _uType: u32, /* MESSAGEBOX_STYLE */
) -> u32 /* MESSAGEBOX_RESULT */ {
    stub!(0)
}

#[win32_derive::dllexport]
pub fn MessageBoxW(
    _ctx: &mut Context,
    _hWnd: HWND,
    _lpText: Ptr<u16>,    /* WSTR */
    _lpCaption: Ptr<u16>, /* WSTR */
    _uType: u32,          /* MESSAGEBOX_STYLE */
) -> u32 /* MESSAGEBOX_RESULT */ {
    stub!(0)
}

#[win32_derive::dllexport]
pub fn SetMenu(_ctx: &mut Context, _hWnd: HWND, _hMenu: HMENU) -> bool {
    stub!(true) // success
}

#[win32_derive::dllexport]
pub fn SetTimer(
    _ctx: &mut Context,
    _hWnd: HWND,
    _nIDEvent: u32,
    _uElapse: u32,
    _lpTimerFunc: Ptr<()>, /* TIMERPROC */
) -> u32 {
    stub!(0) // fail
}

// XXX: cdecl
#[win32_derive::dllexport]
pub fn wsprintfW(
    _ctx: &mut Context,
    _param0: Ptr<u16>, /* WSTR */
    _param1: Ptr<u16>, /* WSTR */
) -> i32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn wsprintfA(ctx: &mut Context) -> i32 {
    /// Documented maximum output of wsprintf, including the nul.
    const MAX_LEN: usize = 1024;

    // Cdecl varargs: declared with no args so the wrapper leaves the caller's
    // stack alone; read everything manually.
    // [esp] = return addr, [esp+4] = dst, [esp+8] = fmt, [esp+12...] = args.
    let esp = ctx.cpu.regs.esp;
    let dst = ctx.memory.read::<u32>(esp + 4);
    let fmt_addr = ctx.memory.read::<u32>(esp + 8);
    let fmt = ctx.memory.read_str(fmt_addr).to_owned();
    let mut arg_addr = esp + 12;

    let bytes = fmt.as_bytes();
    let mut out: Vec<u8> = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i];
        i += 1;
        if c != b'%' {
            out.push(c);
            continue;
        }
        let mut left = false;
        let mut zero = false;
        loop {
            match bytes.get(i) {
                Some(b'-') => {
                    left = true;
                    i += 1;
                }
                Some(b'0') => {
                    zero = true;
                    i += 1;
                }
                Some(b'#') => i += 1,
                _ => break,
            }
        }
        let mut width = 0usize;
        while let Some(&d @ b'1'..=b'9') = bytes.get(i) {
            width = width * 10 + (d - b'0') as usize;
            i += 1;
            while let Some(&d @ b'0'..=b'9') = bytes.get(i) {
                // Clamped because the width sizes an allocation here, and a
                // format string can ask for gigabytes of padding.
                width = (width * 10 + (d - b'0') as usize).min(MAX_LEN);
                i += 1;
            }
        }
        if bytes.get(i) == Some(&b'.') {
            i += 1;
            while matches!(bytes.get(i), Some(b'0'..=b'9')) {
                i += 1;
            }
        }
        while matches!(bytes.get(i), Some(b'l') | Some(b'h')) {
            i += 1;
        }
        let spec = *bytes.get(i).unwrap_or(&b'%');
        i += 1;
        let mut next_arg = || {
            let value = ctx.memory.read::<u32>(arg_addr);
            arg_addr += 4;
            value
        };
        let formatted: Vec<u8> = match spec {
            b'%' => vec![b'%'],
            b'd' | b'i' => format!("{}", next_arg() as i32).into_bytes(),
            b'u' => format!("{}", next_arg()).into_bytes(),
            b'x' => format!("{:x}", next_arg()).into_bytes(),
            b'X' => format!("{:X}", next_arg()).into_bytes(),
            b'c' => vec![next_arg() as u8],
            b's' => {
                let addr = next_arg();
                ctx.memory.read_str(addr).as_bytes().to_vec()
            }
            _ => {
                // Consume the arg anyway: skipping it would shift every
                // argument after this one.
                next_arg();
                log::warn!("wsprintfA: unhandled %{}", spec as char);
                vec![b'%', spec]
            }
        };
        if formatted.len() < width {
            let pad = if zero && !left { b'0' } else { b' ' };
            let padding = std::iter::repeat(pad).take(width - formatted.len());
            if left {
                out.extend(formatted);
                out.extend(padding);
            } else {
                out.extend(padding);
                out.extend(formatted);
            }
        } else {
            out.extend(formatted);
        }
    }

    // The real wsprintfA writes at most 1024 characters including the nul, and
    // callers size their buffers for that.
    out.truncate(MAX_LEN - 1);
    ctx.memory[dst..][..out.len()].copy_from_slice(&out);
    ctx.memory.write::<u8>(dst + out.len() as u32, 0);
    out.len() as i32
}
