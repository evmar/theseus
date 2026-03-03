
#include "wasm.h"
#include "kernel32.h"

typedef struct {
    u32 eax, ecx, edx, ebx;
    u32 esp;
} Regs;

static Regs* regs = (Regs*)0x1000;

inline void push(u32 x) {
    regs->esp -= 4;
    *((u32*)regs->esp) = x;
}

inline void stdcall_GetStdHandle() {
    u32* stack = (u32*)regs->esp;
    regs->eax = kernel32_GetStdHandle(stack[0]);
    regs->esp += 4;
}

inline void stdcall_WriteFile() {
    u32* stack = (u32*)regs->esp;
    regs->eax = kernel32_WriteFile(stack[0], stack[1], stack[2], stack[3], stack[4]);
    regs->esp += 5*4;
}

export void x401000(int x) {
    regs->esp = 0x2000;
    u8* mem  = (u8*)0x402000;
    mem[0] = 'h';

    /*
    00401000 push 0FFFFFFF5h
    00401002 call dword ptr ds:[402058h]
    00401008 xor ecx,ecx
    0040100a push ecx
    0040100b push ecx
    0040100c push 6
    0040100e push 402000h
    00401013 push eax
    00401014 call dword ptr ds:[40205Ch]
    0040101a ret
    */

    push(-1);
    stdcall_GetStdHandle();
    regs->ecx ^= regs->ecx;
    push(regs->ecx);
    push(regs->ecx);
    push(6);
    push(0x402000);
    push(regs->eax);
    stdcall_WriteFile();
}
