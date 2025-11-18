# compatible-libducohash
Minimal changes to libducohash

Fixed compile time error.
Contains a warning suppression, which is fixed in the [other version](https://github.com/profpandaa/libducohash).

# How to compile
Run `cargo build --release` in the directory of the repository and get the binary from `target/release`. The binary is only compatible for the platform you compile it on. For cross-compilation check out the [cross project](https://github.com/cross-rs/cross).
