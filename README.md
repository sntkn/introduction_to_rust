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

### diesel

ORM ＋クエリビルダー

```bash
cargo install diesel_cli --no-default-features --features sqlite-bundled
diesel setup

# migration
diesel migration generate create_posts
diesel migration run # migrate
```

### sqlx

コンパイルチェックと非同期に対応

```bash
cargo install sqlx-cli


# migration
sqlx migrate add label
sqlx migrate run
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

| -      | 型   | 長     | 代入    |
| ------ | ---- | ------ | ------- |
| tuple  | 複合 | 固定長 | 不可    |
| array  | 単一 | 固定長 | 可(mut) |
| vector | 単一 | 可変長 | 可(mut) |

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

slice 参照を使えばどんな配列でも処理できる

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

### iterator

```rust

 v.into_iter() // iterator (v が破棄される)
 for i in vec![10, 20, 30] // into_iter と同じ
 v.iter() // reference iterator（参照なので破棄されない）
 for i in &vec![10, 20, 30] // iter と同じ
 v.iter_mut() // mutating iterator
 for i in &mut vec![10, 20, 30] // iter_mut と同じ
```

#### iterator generator

- filter
- map
- enumerate -> `for (i, ch) in arr.into_iter().enumerate()`
- any
- all
- count
- sum
- min/max
- collect -> `arr.into_iter().collect::<Vec<i32>>();`

### 基本の型

#### Option

データが存在するか存在しないか。

```rust
pub enum Option<T> {
  None,
  Some(T),
}
```

#### Result

処理結果が成功かエラーか。

```rust
pub enum Result<T, E> {
  Ok(T),
  Err(E),
}
```

Result 型の処理方法は以下のものがある

- match .... 場合分けして処理する
- unwrap_or() .... or の値を返す
- unwrap() .... Err を無視する
- and_then(f) .... Ok だったら関数 f を実行する
- ? .... Err の場合は呼び出し元に Err を返却する

#### Vec

要素の増減が可能な配列。

#### Box

ヒープ領域に値を格納する。

- コンパイル時にサイズがわからない型
- 大きなサイズの方の値を渡すのにポインタで渡す
- 共通のトレイトを実装したさまざまな型を画一的にポインタで扱う

### Attribute

`#[xxxx]` を記述する

```rust

#[derive(Eq, PartialEq)]
struct A(i32); // 一致比較 A(0) == A(1)

#[derive(PartialEq, PartialOrd)]
struct B(f32); // 代償比較 B(0.1) > B(0.2)

#[derive(Copy, Clone)]
struct C; // コピー, not move, a = c; b = c;

#[derive(Clone)]
struct D; // クローン d.clone

#[derive(Debug)]
struct E; // デバッグプリント

#[derive(Default)]
struct F; // default F::default();
```

### error

便利なエラーハンドリングの crate

#### anyhow

主に自作アプリケーション用途で手軽にエラーハンドリング  
Result 型をラップして、anyhow::Error を隠蔽する。  
bail!, ensure! といった早期リターンもできる

## cargo generate

テンプレートリポジトリからプロジェクトを始める  
以下は wasm プロジェクトの始めかた

```bash
cargo install cargo-generate
cargo generate --git https://github.com/rustwasm/wasm-pack-template

curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
cd project
wasm-pack build
npm init wasm-app www
cd www
npm install
npm run start
```

## wasm

### wasm-bindgen

JavaScript と Rust のバインディングを行うクレート。  
js-sys, web-sys もその一部。
