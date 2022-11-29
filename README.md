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

# show document on browser
cargo doc --open

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

## 基本構文

### 束縛

for も含めて `{}` の中はスコープが違う  
下記の場合、1 回ごと x がリセットされる

```rust
    let x = 5;
    for n in 1..11 {
        let x = x + 1;
        println!("{}", x);
    }

    println!("{}", x);
```

### Collection

-|型|長|代入
---|---|---|---
tuple|複合|固定長|不可
array|単一|固定長|可(mut)
vector|単一|可変長|可(mut)

```rust
let t: (i32, f64, &str) = (100, 1.01, "hundred") // tuple
let a: [i32; 5] = [1, 2, 3, 4, 5]; // array
let v: Vec<i32> = vec![1,2,3]; // vector
```
