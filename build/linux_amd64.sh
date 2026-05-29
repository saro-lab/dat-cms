rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
mv -f ./target/x86_64-unknown-linux-musl/release/dat-cms ./target/dat_cms_linux_amd64
echo "$PWD/target/dat_cms_linux_amd64"
