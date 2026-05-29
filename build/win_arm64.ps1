rustup target add aarch64-pc-windows-msvc
cargo build --release --target aarch64-pc-windows-msvc
mv "./target/aarch64-pc-windows-msvc/release/dat-cms.exe" "./target/dat_cms_arm64.exe" -Force
