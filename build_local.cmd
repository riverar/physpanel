SET RUSTFLAGS=--remap-path-prefix=%USERPROFILE%=.\ --remap-path-prefix=%CD%=.\
cargo build --target x86_64-pc-windows-msvc --release
cargo build --target aarch64-pc-windows-msvc --release
SET RUSTFLAGS=