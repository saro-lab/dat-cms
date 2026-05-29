rustup target add x86_64-pc-windows-msvc
cargo build --release --target x86_64-pc-windows-msvc
mv "./target/x86_64-pc-windows-msvc/release/dat-cms.exe" "./target/dat_cms_win_amd64.exe" -Force
