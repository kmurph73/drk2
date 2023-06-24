cargo build --target aarch64-apple-ios --release
rm /Users/kmurph/Downloads/SDL-release-2.26.5/Xcode-iOS/Demos/src/libdrrust.a
mv ./target/aarch64-apple-ios/release/libdrrust.a /Users/kmurph/Downloads/SDL-release-2.26.5/Xcode-iOS/Demos/src