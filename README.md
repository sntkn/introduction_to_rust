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

### 所有権

```rust
    let s1 = String::from("hello");
    let s2 = s1; // ここで所有権が s2 に移動して s1 は破棄される（ムーブ）
    let s2 = s1.clone(); // deep copy すれば両方有効

    let x = "hello";
    let y = x; // リテラルはコピーしてスタックに積まれるためどちらも有効

```

#### borrow

```rust
  fn change(some_string: &String) {
    some_string.push_str(", world");  // Error. 借用は immutable
  }

  // 変数宣言、呼び出し元、呼び出し先で mut つける
  fn change(some_string: &mut String) {
    some_string.push_str(", world"); // mut をつけることで mutable
  }

  let r1 = &mut s;
  let r2 = &mut s; // ただし、一つしか借用できない

  // スコープ内では借用できる
  let mut s = "hello";
  {
    let r1 = addworld(&mut s); // r1=hello world
  } // ここでスコープ抜ける
  let r2 = addworld(&mut s); // r2,s=hello world world
```

### struct

```rust
#[derive(Debug)] // debug
struct Rectangle {
    width: u32,
    height: u32,
}

// method 記法
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {:?}", rect1); // 構造体を展開
}
```

### Generics

```rust

enum Option<T> {
  Some(T),
  None,
}
// utility fn
x.is_some(), x.is_none(), x.unwrap()

enum Result<T, E> {
  Ok(T),
  Err(E)
}
// utility fn
x.is_ok(), x.is_err(), x.unwrap(),
```

### slice

slice参照を使えばどんな配列でも処理できる

```rust
// slice reference &[i32]
fn min(arr: &[i32]) -> i32 {
  let mut minimum = arr[0];
  for i in 1..arr.len() {
    if arr[i] < minimum { minimum = arr[i]; }
    minimum
  }
}
print!("{}", min(&[23, 17]));
print!("{}", min(&vec![55, 22, 33, 44]));
```
