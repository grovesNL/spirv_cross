#!/bin/bash

# Install Android NDK and create standalone toolchain
export NDK_HOME=$TRAVIS_BUILD_DIR/android-ndk-r16b
export ANDROID_TOOLCHAIN=$TRAVIS_BUILD_DIR/android-toolchain
export PATH=$PATH:$ANDROID_TOOLCHAIN/bin
export ANDROID_TARGET=arm-linux-androideabi
export CXX_arm_linux_androideabi=$ANDROID_TARGET-gcc++
rm -rf $NDK_HOME
curl -L http://dl.google.com/android/repository/android-ndk-r16b-linux-x86_64.zip -O
unzip -oq android-ndk-r16b-linux-x86_64.zip
rm android-ndk-r16b-linux-x86_64.zip
python $NDK_HOME/build/tools/make_standalone_toolchain.py --arch arm --api 21 --stl=libc++ --install-dir $ANDROID_TOOLCHAIN

# Install Rust
export RUSTFLAGS="-C link-args=-no-pie -C link-args=-Wl,-Bsymbolic"
export PATH=$HOME/.cargo/bin/:$PATH
curl https://sh.rustup.rs -o rustup.sh
sh rustup.sh -y
rustup target add arm-linux-androideabi
cargo install cargo-apk

# Build GLSL example
cd examples
export CPATH=$NDK_HOME/sources/cxx-stl/llvm-libc++/include:$NDK_HOME/sysroot/usr/include/arm-linux-androideabi:$JAVA_HOME/include:$JAVA_HOME/include/linux:$NDK_HOME/sysroot/usr/include:$CPATH
cargo apk build --bin glsl
