rustup target add aarch64-unknown-linux-musl
cargo build --release --target aarch64-unknown-linux-musl
mv -f ./target/aarch64-unknown-linux-musl/release/dat-cms ./target/dat_cms_linux_arm64
echo "$PWD/target/dat_cms_linux_arm64"
