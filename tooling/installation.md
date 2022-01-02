# Installation

Download the Rust compiler and the Cargo package manager from https://www.rust-lang.org/learn/get-started.

## Linux / Mac

```bash
# the default way using Rustup
curl https://sh.rustup.rs -sSf | sh

# also possible using Nix or another package manager
```

## Windows

1. Make sure the C++ requirements are installed. Download the C++ build-tools from: https://visualstudio.microsoft.com/visual-cpp-build-tools/
2. Download the latest Rust from: https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe

```bash
# or, from WSL (similar to Linux/Mac)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# or, with Scoop/Chocolatey
scoop install rust
choco install rust
```

## Verify install

```bash
# check the Cargo version
cargo version

# check Rustup version
rustup --version
```

## Update Rust

```bash
# update Rust to the latest version
rustup update
```

## Code editor support

Plugins for:
- VScode: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust
- Vim: https://github.com/rust-lang/rust.vim

```bash
# install Rust code formatter
rustup component add rustfmt # installed by default?
```
