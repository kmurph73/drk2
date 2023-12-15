PRJ=/Users/kmurph/code/drk2-android-prj

cargo build --target aarch64-linux-android --release
rm $PRJ/app/jni/src/libdrrust.a 2> /dev/null
mv ./target/aarch64-linux-android/release/libdrrust.a $PRJ/app/jni/src
rm $PRJ/app/src/main/assets/resources/skyline-packer-output.png 2> /dev/null
cp resources/skyline-packer-output.png $PRJ/app/src/main/assets/resources
rm $PRJ/app/src/main/assets/resources/aboot.png 2> /dev/null
cp aboot.png $PRJ/app/src/main/assets/resources