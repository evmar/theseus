#!/bin/bash

set -e

# munge a dosbox LOGCPU.TXT into a form for diffing against theseus --trace output.
#
# input looks like
# 0823:0000068E  mov  ah,30                                             EAX:00000000 EBX:00000000 ECX:000000FF EDX:00000813 ESI:0000068E EDI:00000800 EBP:0000091C ESP:00000800 DS:0813 ES:0813 FS:0000 GS:0000 SS:0A71 CF:0 ZF:0 SF:0 OF:0 AF:0 PF:0 IF:1

replacements1=(
    # remove interrupts
    -e '/^F000:/d'
    # remove ... other interrupts?
    -e '/^C000:/d'
    # trim segments; 813 is default, 823 in exe
    -e 's/^08.3://'
    # wrap registers EAX, ESI, DS
    -e 's/ EAX/\nEAX/'
    -e 's/ ESI/\nESI/'
    -e 's/ DS/\nDS/'
    # wrap flags
    -e 's/ CF:/\nCF:/'
    # remove parity/aux flags
    #-e 's/PF:. //'
    #-e 's/AF:. //'
    # drop all repe, they get printed multiple times for loop
    -e '/repe/d'
    -e '/repne/d'
)

replacements2=(
    # remove instructions
    -e 's/^(0.......)(.*)/\1/'
    # drop flags
    -e '/^CF:/d'
)

exec sed -E "${replacements1[@]}" | sed -E "${replacements2[@]}"
