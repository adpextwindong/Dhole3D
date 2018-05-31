#!/bin/sh

if [ -e SDL2-devel-2.0.8-mingw.tar.gz ] #&& [ -e SDL2-devel-2.0.8-VC.zip ]
then
    rm -rf SDL2-2.0.8/
    rm -rf gnu-mingw
    rm -rf msvc
    tar xzf SDL2-devel-2.0.8-mingw.tar.gz
    #unzip -f SDL2-devel-2.0.8-VC.zip

    mkdir -p gnu-mingw/dll/32
    mkdir -p gnu-mingw/lib/32
    
    mkdir -p gnu-mingw/dll/64
    mkdir -p gnu-mingw/lib/64

    cp SDL2-2.0.8/i686-w64-mingw32/bin/* gnu-mingw/dll/32/
    cp SDL2-2.0.8/i686-w64-mingw32/lib/* gnu-mingw/lib/32/

    cp SDL2-2.0.8/x86_64-w64-mingw32/bin/* gnu-mingw/dll/64/
    cp SDL2-2.0.8/x86_64-w64-mingw32/lib/* gnu-mingw/lib/64/

    #mkdir -p msvc/dll/32
    #mkdir -p msvc/dll/64
    #mkdir -p msvc/lib/32
    #mkdir -p msvc/lib/64
fi