#!/bin/bash

cd "$(dirname "$0")/.."

echo $PWD

rm -rf gen

cargo tauri android init

cp ./android-build/key.properties ./gen/android/
cp ./android-build/build.gradle.kts ./gen/android/app/
