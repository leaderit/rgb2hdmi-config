# !/bin/sh
# for build must have done
# rustup target add x86_64-pc-windows-gnu 
# rustup target add aarch64-unknown-linux-gnu 
# rustup target add x86_64-unknown-linux-gnu 
# cargo install cross --git https://github.com/cross-rs/cross
# and start docker!
NAME=rgb2hdmi-config
ARC_X86=x86_64-apple-darwin
ARC_ARM64=aarch64-apple-darwin
ARC_WIN64=x86_64-pc-windows-gnu
ARC_LINUX64=x86_64-unknown-linux-gnu 

rustup target add x86_64-pc-windows-gnu 
rustup target add aarch64-unknown-linux-gnu 
rustup target add x86_64-unknown-linux-gnu
# rustup update
cargo install cross --git https://github.com/cross-rs/cross

# Build release binary
cargo clean
cargo build --release --target=$ARC_ARM64 
cargo build --release --target=$ARC_X86 
# Combine x86 and arm targets into one executable file
lipo -create -output $NAME-macos target/$ARC_ARM64/release/$NAME target/$ARC_X86/release/$NAME

cargo build --release --target=$ARC_WIN64 
mv target/$ARC_WIN64/release/$NAME.exe $NAME-win-x86_64.exe

# needs docker have started
cross build --release --target=$ARC_LINUX64
mv target/$ARC_LINUX64/release/$NAME $NAME-linux-x86_64
