cargo build --target aarch64-linux-android --release
rm /Users/kmurph/code/android-prj/app/jni/src/libdrrust.a
mv ./target/aarch64-linux-android/release/libdrrust.a /Users/kmurph/code/android-prj/app/jni/src