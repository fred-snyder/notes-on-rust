# Cargo

https://doc.rust-lang.org/cargo/

The Rust package manager.

## Cargo init

```bash
# Create a Rust project in the current folder:
cargo init
```

This will create a `src` folder with a hello world rust file and a `Cargo.toml` file. Also, a `.gitignore` is created.

## Cargo new

```bash
# create new Rust package
cargo new packageName
```


## Cargo run

```bash
# build and run your application
cargo run
```
The build outputs are stored in the `target/debug` folder.


## Cargo build

This command will build (optimized) executables. It builds all the binaries as setup in `Cargo.toml`. Output is stored in the `target/release` folder.

```bash
# build executable
cargo build

# build optimized executable for production
cargo build --release
```
