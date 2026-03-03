#pragma once

#include "wasm.h"

import("host", "console_log") extern void host_console_log(u32, u32);
import("host", "panic") extern void host_panic(u32);
