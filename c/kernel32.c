#include "host.h"

u32 kernel32_GetStdHandle(u32 x) {
    return 1;
}

u32 kernel32_WriteFile(u32 hFile, u32 lpBuffer, u32 n, u32 nr, u32 o) {
    if (hFile == 1) {
        host_console_log(lpBuffer, n);
    } else {
        host_panic((u32)"writefile");
    }
    return 1;
}
