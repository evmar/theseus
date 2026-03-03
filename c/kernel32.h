#include "wasm.h"

u32 kernel32_GetStdHandle(u32 x);
u32 kernel32_WriteFile(u32 hFile, u32 lpBuffer, u32 n, u32 nr, u32 o);

static inline void stdcall_GetStdHandle() {
    u32* stack = (u32*)regs->esp;
    regs->eax = kernel32_GetStdHandle(stack[0]);
    regs->esp += 4;
}

static inline void stdcall_WriteFile() {
    u32* stack = (u32*)regs->esp;
    regs->eax = kernel32_WriteFile(stack[0], stack[1], stack[2], stack[3], stack[4]);
    regs->esp += 5*4;
}
