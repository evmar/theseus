use runtime::*;
pub fn x00401000() -> Option<u32> {
    unsafe {
        // 00401000 push ebp
        push(REGS.ebp);
        // 00401001 mov ebp,esp
        REGS.ebp = REGS.esp;
        // 00401003 and esp,0FFFFFFF0h
        REGS.esp &= 0xfffffff0u32;
        // 00401006 sub esp,10h
        REGS.esp -= 0x10u32;
        // 00401009 call 00401015h
        return call(0x401015);
    }
}
