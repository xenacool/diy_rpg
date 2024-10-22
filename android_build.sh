#!/usr/bin/env bash

RELEASE_MODE=${1}
LIB_FOLDER="debug"

# build to Android target
if [ "${RELEASE_MODE}" = "--release" ]; then
    LIB_FOLDER="release"
    cargo so b --lib --target aarch64-linux-android ${RELEASE_MODE}
else
    RUST_BACKTRACE=full RUST_LOG=wgpu_hal=debug cargo so b --lib --target aarch64-linux-android
fi

# copy .so files to jniLibs folder
ARM64="android/app/libs/arm64-v8a"
ARMv7a="android/app/libs/armeabi-v7a"

if [ ! -d "$ARM64" ]; then
    mkdir "$ARM64"
fi
if [ ! -d "$ARMv7a" ]; then
    mkdir "$ARMv7a"
fi

cp target/aarch64-linux-android/${LIB_FOLDER}/libdiy_rpg.so "${ARM64}/libdiy_rpg.so"