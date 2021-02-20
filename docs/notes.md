#About wsl2
Its windows 10 build in linux vm.

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustc --print target-list
rustup target add x86_64-pc-windows-msvc
rustup target add i686-pc-windows-msvc
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup toolchain install stable-i686-pc-windows-msvc
rustup target add i686-pc-windows-gnu
 apt install mingw-w64 -y
cargo build --target i686-pc-windows-msvc

note