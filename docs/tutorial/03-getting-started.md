# Provenance Smart Contract Tutorial

## Getting Started

In this section we will set up a development environment for building smart contracts.

## Installation

Ensure that the rust [installer](https://rustup.rs/) is available.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Ensure the stable toolchain is the default.

```bash
rustup default stable
```

Add the `wasm32` target.

```bash
rustup target add wasm32-unknown-unknown
```

Install the `cargo-generate` crate.

```bash
cargo install cargo-generate --features vendored-openssl
```

## Updates

Every so often, the Rust toolchain should be updated

```bash
# check for udpates
rustup check
# update
rustup update stable
```

## Setting up an IDE

An IDE is recommended to help learn syntax, especially when just starting out with Rust.

There are two recommended editors for smart contract development.

### Visual Studio Code

- [Download](https://code.visualstudio.com/download)
- [Rust Plugin](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)

### Intellij IDEA

- [Download](https://www.jetbrains.com/idea/)
- [Rust Plugin](https://intellij-rust.github.io/)

Neither is better or worse than the other. Choose based on personal preference.

### Up Next

To generate a smart contract project using a template, proceed to the [Project](04-project.md)
section.
