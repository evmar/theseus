use crate::{kernel32::state, stub};

#[win32_derive::dllexport]
pub fn GetEnvironmentStrings() -> u32 {
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
    // TODO: if available, this ends up hitting a jmp table when parsing
    // stub!(state().environ.get())
    stub!(0)
}

#[win32_derive::dllexport]
pub fn GetEnvironmentStringsW() -> u32 {
    // TODO: if available, this ends up getting parsed with unimplemented nls functions
    // stub!(state().environ.get())
    stub!(0)
}

#[win32_derive::dllexport]
pub fn GetEnvironmentVariableA(_lpName: u32, _lpBuffer: u32, _nSize: u32) -> u32 {
    stub!(state().environ.get())
}

#[win32_derive::dllexport]
pub fn FreeEnvironmentStringsA(_penv: u32) -> bool {
    //sys.memory().process_heap.free(sys.mem(), penv);
    stub!(true) // success
}

#[win32_derive::dllexport]
pub fn FreeEnvironmentStringsW(_penv: u32) -> bool {
    //sys.memory().process_heap.free(sys.mem(), penv);
    stub!(true) // success
}
