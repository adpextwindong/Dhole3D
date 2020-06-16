#!/bin/sh

VERSION="2.0.12"

MINGW_FILE="SDL2-devel-${VERSION}-mingw.tar.gz"
VC_FILE="SDL2-devel-${VERSION}-VC.zip"

if [ -e $MINGW_FILE ] || [ -e $VC_FILE ]
then

    if [ -e $MINGW_FILE ]
    then
        rm -rf SDL2-${VERSION}/
        tar xzf $MINGW_FILE

        rm -rf gnu-mingw
        mkdir -p gnu-mingw/dll/32
        mkdir -p gnu-mingw/lib/32

        mkdir -p gnu-mingw/dll/64
        mkdir -p gnu-mingw/lib/64

        cp -r SDL2-${VERSION}/i686-w64-mingw32/bin/* gnu-mingw/dll/32/
        cp -r SDL2-${VERSION}/i686-w64-mingw32/lib/* gnu-mingw/lib/32/

        cp -r SDL2-${VERSION}/x86_64-w64-mingw32/bin/* gnu-mingw/dll/64/
        cp -r SDL2-${VERSION}/x86_64-w64-mingw32/lib/* gnu-mingw/lib/64/


    fi

    if [ -e $VC_FILE ]
    then
        echo "Handling VC files"

        rm -rf SDL2-${VERSION}/
        unzip $VC_FILE

        rm -rf msvc
        mkdir -p msvc/dll/32
        mkdir -p msvc/lib/32

        mkdir -p msvc/dll/64
        mkdir -p msvc/lib/64

        cp -r SDL2-${VERSION}/lib/x64/SDL2.dll msvc/dll/64
        cp -r SDL2-${VERSION}/lib/x86/SDL2.dll msvc/dll/32

        cp -r SDL2-${VERSION}/lib/x64/SDL2.lib msvc/lib/64
        cp -r SDL2-${VERSION}/lib/x86/SDL2.lib msvc/lib/32

    fi
else
    echo "Devel tar/zips not found"
fi
