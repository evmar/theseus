use runtime::{Cont, Machine, ret};

pub fn x0040a3b4(m: &mut Machine) -> Cont {
    // code is like
    //   mov edi, 0x403a3b4
    //   call edi
    // ...
    //   403a3b2  lodsd edi
    //   403a3b3  mov cl,0x60
    //   403a3b5  or eax,eax
    // where 403ab4 is in the middle of that?
    // maybe some unpacker failure?
    // It's in directsound shutdown anyway.
    #[allow(static_mut_refs)]
    ret(m, 0)
}
