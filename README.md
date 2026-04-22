![GitHub CI](https://github.com/jedisct1/rust-xoodyak/workflows/Rust/badge.svg)

# Xoospark for Rust

This is a Rust implementation of Xoospark, a cryptographic primitive that can be used for hashing, encryption, MAC computation and authenticated encryption. Xoospark is based on the Sparkle permutation (SparkleP) within the Cyclist mode of operation.

* `no_std`-friendly
* Lightweight
* Can be compiled to WebAssembly/WASI
* Session support
* Safe Rust interface
* AEAD with attached and detached tags
* In-place encryption
* Ratcheting
* Variable-length output hashing, authentication
* `squeeze_more()`, `absorb_more()` for streaming.

# [API documentation](https://docs.rs/xoodyak)
