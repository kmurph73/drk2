cargo build --target aarch64-linux-android --release
rm /Users/kmurph/code/android-prj/app/jni/src/libdrrust.a
mv ./target/aarch64-linux-android/release/libdrrust.a /Users/kmurph/code/android-prj/app/jni/src
rm /Users/kmurph/code/android-prj/app/src/main/assets/resources/skyline-packer-output.png
cp resources/skyline-packer-output.png /Users/kmurph/code/android-prj/app/src/main/assets/resources
rm /Users/kmurph/code/android-prj/app/src/main/assets/resources/aboot.png
cp aboot.png /Users/kmurph/code/android-prj/app/src/main/assets/resources