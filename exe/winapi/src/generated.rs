use runtime::{REGS, push};
pub fn x00401000() -> Option<u32> {
    unsafe {
        // 00401000 push 0FFFFFFF5h
        push(0xfffffff5u32);
        // 00401002 call dword ptr ds:[402058h]
        todo!("KERNEL32.dll:GetStdHandle");
        // 00401008 xor ecx,ecx
        REGS.ecx ^= REGS.ecx;
        // 0040100a push ecx
        push(REGS.ecx);
        // 0040100b push ecx
        push(REGS.ecx);
        // 0040100c push 6
        push(0x6u32);
        // 0040100e push 402000h
        push(0x402000u32);
        // 00401013 push eax
        push(REGS.eax);
        // 00401014 call dword ptr ds:[40205Ch]
        todo!("KERNEL32.dll:WriteFile");
        // 0040101a ret
        return None;
    }
}
