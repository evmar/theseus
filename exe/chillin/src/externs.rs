use runtime::{Cont, ret};

pub fn x0040a3b4() -> Cont {
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
    ret(0)
}
