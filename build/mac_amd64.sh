rustup target add x86_64-apple-darwin
cargo build --release --target x86_64-apple-darwin
mv -f ./target/x86_64-apple-darwin/release/dat-cms ./target/dat_cms_mac_amd64
