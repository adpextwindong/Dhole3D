#!/bin/bash
if [ "$(uname -o)" = "Cygwin" ]; then
    echo
    echo "Fetching Windows SDL2 Deps"
    echo

    if [ -f README-SDL.txt ]; then
        rm README-SDL.txt
    fi;
    if [ -f SDL2.dll ]; then
        rm SDL2.dll
    fi;

    wget --no-verbose https://www.libsdl.org/release/SDL2-2.0.8-win32-x64.zip;
    unzip SDL2-2.0.8-win32-x64.zip;
    rm SDL2-2.0.8-win32-x64.zip*;
fi;