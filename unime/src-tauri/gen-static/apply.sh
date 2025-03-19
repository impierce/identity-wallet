# Copies all files tracked by git (/gen-static) to the generated resources (/gen)

cp -v ./android/app/build.gradle.kts ../gen/android/app/build.gradle.kts
cp -v ./android/app/proguard-rules.pro ../gen/android/app/proguard-rules.pro
cp -v ./android/app/tauri.properties ../gen/android/app/tauri.properties
cp -v ./android/app/src/main/AndroidManifest.xml ../gen/android/app/src/main/AndroidManifest.xml
cp -v ./apple/ExportOptions.plist ../gen/apple/ExportOptions.plist
cp -v ./apple/unime_iOS/Info.plist ../gen/apple/unime_iOS/Info.plist
