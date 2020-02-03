#!/bin/sh

crate=fluidlite-lib
name=fluidlite

android_api=16
android_targets="armv7-linux-androideabi aarch64-linux-android i686-linux-android x86_64-linux-android"
#desktop_targets="arm-unknown-linux-gnueabihf arm-unknown-linux-gnueabi armv7-unknown-linux-gnueabi armv7-unknown-linux-gnueabihf aarch64-unknown-linux-gnu i686-unknown-linux-gnu x86_64-unknown-linux-gnu"
#desktop_targets="i686-unknown-linux-gnu x86_64-unknown-linux-gnu"
targets="${android_targets} ${desktop_targets}"

workdir=$PWD

rm -rf $workdir/prebuilt
mkdir -p $workdir/prebuilt

cd $workdir/$crate
#cargo clean

for target in $targets; do
    if echo $target | grep android > /dev/null; then
        cargo="cargo ndk --android-platform $android_api --target $target -- build"
    else
        cargo="cargo build --target $target"
    fi
    $cargo
    $cargo --release
done

cd $workdir

for target in $targets; do
    for profile in debug release; do
        cd target/$target/$profile/build/$crate-*/out/build
        tar -czf $workdir/prebuilt/lib${name}_${target}_${profile}.tar.gz lib${name}.{a,so}
        cd $workdir
    done
done
