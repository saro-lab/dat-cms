rustup target add aarch64-apple-darwin
cargo build --release --target aarch64-apple-darwin
mv -f ./target/aarch64-apple-darwin/release/dat-cms ./target/dat_cms_mac_arm64
