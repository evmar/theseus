#!/bin/bash

set -e

# projects are roughly in complexity order, with the simple ones first.
projects=(
    winapi
    thread
    zig_hello

    basicdd

    chillin-unpack
    chillin

    mine

    mofo-unpack

    dos/animate-unpack
    dos/animate-unpack-2
    dos/animate

    dos/magixfly

    sbaitso/read
    sbaitso/sbtalker
)

for project in "${projects[@]}"; do
    echo "$ out/$project/translate.sh"
    "out/$project/translate.sh"
done
