#pragma once

#include <stdint.h>

typedef uint32_t u32;
typedef uint8_t u8;

#define import(module, name) __attribute__((import_module(module))) __attribute__((import_name(name)))
#define export __attribute__((visibility("default")))

typedef struct {
    u32 eax, ecx, edx, ebx;
    u32 esp;
} Regs;

static Regs* regs = (Regs*)0x1000;

static inline void push(u32 x) {
    regs->esp -= 4;
    *((u32*)regs->esp) = x;
}
