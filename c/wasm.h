#pragma once

#include <stdint.h>

typedef uint32_t u32;
typedef uint8_t u8;

#define import(module, name) __attribute__((import_module(module))) __attribute__((import_name(name)))
#define export __attribute__((visibility("default")))
