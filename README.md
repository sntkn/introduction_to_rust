# introduction_to_rust

Rust 入門

## Instration

```bash
brew install rustup-init
rustup-init
```

## プロジェクトの作成と実行

```bash
cargo new [project-name]

cargo run
```

## cargo

```bash
# check compile
cargo check

# build
cargo build

# dependency package(crate)
cargo add [crate name]

# format
cargo fmt

# code check
cargo clippy

# cargo make
cargo install cargo-make #install
cargo make [command] # execute

```

## database

```bash
cargo install diesel_cli --no-default-features --features sqlite-bundled
diesel setup

# migration
diesel migration generate create_posts
diesel migration run # migrate
```

## aws

### lambda

```bash
php3 install cargo-lambda
cargo lambda new [project name]
```

## Foreign Function interface

```bash
cargo install maturin
maturin new [project name] --bindings pyo3

maturin develop
```

## WebAssembly

wasm 向けビルド環境

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk

cargo new [project name]
cargo add yew wasm-bindgen-futures gloo-net
```
