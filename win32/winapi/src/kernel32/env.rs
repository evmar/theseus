use runtime::Context;

use crate::{Ptr, kernel32::lock, stub};

#[win32_derive::dllexport]
pub fn GetEnvironmentStrings(_ctx: &mut Context) -> u32 {
    /*
        // Yes, this function without "A" suffix exists:
        // https://devblogs.microsoft.com/oldnewthing/20130117-00/?p=5533
        let mut measure = EncoderAnsi::new(&mut []);
        let state = get_state(sys);
        encode_env(&mut measure, &state.env);
        let len = measure.status().unwrap_err();

        let addr = sys.memory().process_heap.alloc(sys.mem(), len as u32);

        let mut encoder = EncoderAnsi::from_mem(sys.mem(), addr, len as u32);
        encode_env(&mut encoder, &state.env);
        encoder.status().unwrap();
    */
    // An empty environment block: a list of nul-terminated strings,
    // terminated by an extra nul.
    let kernel32 = lock();
    let addr = kernel32.process_heap.alloc(&mut _ctx.memory, 2);
    _ctx.memory[addr..][..2].fill(0);
    addr
}

#[win32_derive::dllexport]
pub fn GetEnvironmentStringsW(_ctx: &mut Context) -> u32 {
    // Returning 0 pushes the CRT towards the ANSI fallback (GetEnvironmentStrings).
    stub!(0)
}

#[win32_derive::dllexport]
pub fn GetEnvironmentVariableA(
    _ctx: &mut Context,
    _lpName: Ptr<u8>,
    _lpBuffer: Ptr<u8>,
    _nSize: u32,
) -> u32 {
    stub!(lock().environ.get())
}

#[win32_derive::dllexport]
pub fn FreeEnvironmentStringsA(_ctx: &mut Context, _penv: Ptr<u8>) -> bool {
    //sys.memory().process_heap.free(sys.mem(), penv);
    stub!(true) // success
}

#[win32_derive::dllexport]
pub fn FreeEnvironmentStringsW(_ctx: &mut Context, _penv: Ptr<u16>) -> bool {
    //sys.memory().process_heap.free(sys.mem(), penv);
    stub!(true) // success
}
