podman machine start
cross build --target aarch64-unknown-linux-gnu --release
cross build --target arm-unknown-linux-gnueabihf --release
cross build --target armv7-unknown-linux-gnueabihf --release
cross build --target armv7-unknown-linux-gnueabi --release
cross build --target x86_64-pc-windows-msvc --release
cross build --target x86_64-unknown-linux-gnu --release