use std::{
    cell::RefCell,
    convert::{TryFrom, TryInto},
    fmt::Debug,
    ops::Neg,
    rc::{Rc, Weak},
};

mod module_hello;

fn main() {
    func_ex_print_some(func_ex_div_some(10, 5));
    func_ex_print_some(func_ex_div_some(10, 0));
    func_ex_print_some_match(func_ex_div_some(10, 5));
    func_ex_print_some_match(func_ex_div_some(10, 0));
    func_ex_print_result(func_ex_div_result(10, 5));
    func_ex_print_result(func_ex_div_result(10, 0));
    crate::module_hello::print_hello();
    let s = "Hello".to_string();
    myprint(&s); // リファレンスとして渡す

    // イミュータブルな変数は２つリファレンスすることはできない
    let mut s = "Hello".to_string();
    let s_ref1 = &mut s;
    //let s_ref2 = &mut s; // mutable more than once at a time
    myclear(s_ref1);

    let taro = Person::new(String::from("taro"), 10);
    println!("{}({})", taro.name, taro.age_incr(5));

    let mut v: Vec<EnumExample> = vec![];
    v.push(EnumExample::StructPerson {
        name: String::from("struct taro"),
    });
    for e in &v {
        if let EnumExample::StructPerson { name } = e {
            println!("{}", name) // match でも取り出せる
        }
    }

    let node1 = Rc::new(RefCell::new(Node {
        data: 1,
        child: None,
    }));
    let node2 = Rc::new(RefCell::new(Node {
        data: 2,
        child: None,
    }));
    node1.borrow_mut().child = Some(Rc::downgrade(&node2)); //Rc::downgrade()は弱結合化。Rc::clone()は強結合
    node2.borrow_mut().child = Some(Rc::downgrade(&node2));
    // Weak::upgrade() は強結合化

    let rect = Rectangle {
        width: 1.0,
        height: 2.0,
    };
    println!("rec area={}", area(&rect));

    let tria = RightTrianble {
        width: 1.0,
        height: 2.0,
    };
    println!("tria area={}", area2(&tria));

    println!("{}", 1.iabs());
    println!("{}", (-1).iabs());
}

fn _pick(x: &[i32], end: usize) -> &[i32] {
    // let x = [1, 2, 3, 4]; // ローカル変数をリファレンスとして返却できない
    &x[..end]
}

// 複数のリファレンスを含む返却値はライフタイムパラメータとジェネリクスパラメータ（&'a）をつける
fn _pick2<'a, 'b>(x: &'a [i32], y: &'b [i32], end: usize) -> (&'a [i32], &'b [i32]) {
    (&x[..end], &y[..end])
}
fn myprint<T: std::fmt::Display>(msg: &T) {
    // リファレンスとして受け取る
    println!("{}", *msg); // でリファレンスとして実行
}

fn myclear(x: &mut String) {
    // リファレンス＆ミュータブルとして受け取る
    x.clear();
}

fn func_ex_div_some(x: i32, y: i32) -> Option<i32> {
    let ans = if y == 0 { None } else { Some(x / y) };
    ans
}

// Resul<T, E> はTが成功、Eがエラーで返却される
// &'static は＆はリファレンス、'staticはライフタイムパラメータ(コンパイル時に決まっているやつ？)
fn func_ex_div_result(x: i32, y: i32) -> Result<i32, &'static str> {
    if y == 0 {
        Err("div by zero")
    } else {
        Ok(x / y)
    }
}

// Option<T> を引数に渡す型の制約は <T: std::fmt::Display> で指定
fn func_ex_print_some<T: std::fmt::Display>(ans: Option<T>) {
    if let Some(x) = ans {
        println!("{}", x)
    } else {
        println!("None")
    }
}

// 上と同じものを match 式で
fn func_ex_print_some_match<T: std::fmt::Display>(ans: Option<T>) {
    match ans {
        Some(x) => println!("{}", x),
        None => println!("None"),
    }
}

// Result を引数に渡す型の制約が二つ
fn func_ex_print_result<T: std::fmt::Display, E: std::fmt::Display>(ans: Result<T, E>) {
    match ans {
        Ok(res) => println!("{}", res),
        Err(str) => println!("{}", str),
    }
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Person {
        Person { name, age }
    }

    fn age_incr(&self, incr: u8) -> u8 {
        self.age + incr
    }

    // 書き換える場合は &mut self にする
    fn _age_incr_replace(&mut self, incr: u8) {
        self.age += incr;
    }
}

#[derive(Debug)]
struct Parents<'a, 'b> {
    // 複数の構造体を参照で紐付けたい場合はやっぱりラベル付けが必要
    father: &'a Person,
    mother: &'b Person,
}

// impl と Parents, 引数、戻り値に全てライフタイプパラメータをつける
impl<'a, 'b> Parents<'a, 'b> {
    fn new(father: &'a Person, mother: &'b Person) -> Parents<'a, 'b> {
        Parents { father, mother }
    }
}

enum EnumExample {
    StructPerson { name: String }, // enum に構造体を持たせることもできる
    StructParents { name: String },
    // Parents, // 定義済みのstructを設定するのは無理か
}

#[allow(dead_code)]
struct Node {
    data: i32,
    child: Option<Weak<RefCell<Node>>>,
}

trait CalcArea {
    fn calc_area(&self) -> f64;
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl CalcArea for Rectangle {
    fn calc_area(&self) -> f64 {
        self.width * self.height
    }
}

struct RightTrianble {
    width: f64,
    height: f64,
}

impl CalcArea for RightTrianble {
    fn calc_area(&self) -> f64 {
        self.width * self.height * 0.5
    }
}

// T型のトレイト境界の条件をつける
// 複数の場合は＜T： A + B + C> など
fn area<T: CalcArea>(x: &T) -> f64 {
    x.calc_area()
}

// 上記を where で書くこともできる
fn area2<T>(x: &T) -> f64
where
    T: CalcArea,
{
    x.calc_area()
}

// インターフェース的な定義だけでなく実装も書ける
trait PrintHello {
    fn print_hello(&self) {
        println!("hello"); // トレイトに実装
    }
}

struct Test;

impl PrintHello for Test {
    fn print_hello(&self) {
        // 上書きする場合は引数返却を合わせる
        println!("hello world"); // メソッド上書き
    }
}

// trait IAbs<T, S> {
//     fn iabs(self) -> S;
// }
//
// impl IAbs<i32, u32> for i32 {
//     fn iabs(self) -> u32 {
//         if self >= 0 {
//             self as u32
//         } else {
//             -self as u32
//         }
//     }
// }

// 上記の実装でも動くが、本体の方から従属的に決まる型Sをジェネリックパラメータで指定するのは不自然なので
// 下記のように関連型（associated type）を使うとスマート
// <型 as トレイト>::関連型

//trait IAbs {
//    type Output;
//    fn iabs(self) -> <Self as IAbs>::Output;
//}
//
//impl IAbs for i32 {
//    type Output = u32;
//    fn iabs(self) -> <Self as IAbs>::Output {
//        if self >= 0 {
//            self as <Self as IAbs>::Output
//        } else {
//            (-self) as <Self as IAbs>::Output
//        }
//    }
//}

// さらにさまざまなタイプに対応させるには、トレイト境界をたくさんつける
// Sized 値のサイズが決まっている
// PartialOrd 大小比較
// From i8 からの変換
// Into, TryInto 型の変換
trait IAbs {
    type Output;
    fn iabs(self) -> <Self as IAbs>::Output
    where
        Self: Sized + PartialOrd + Neg + From<i8> + TryInto<<Self as IAbs>::Output>,
        <Self as IAbs>::Output: TryFrom<<Self as Neg>::Output>,
        <Self as TryInto<<Self as IAbs>::Output>>::Error: Debug,
        <<Self as IAbs>::Output as TryFrom<<Self as Neg>::Output>>::Error: Debug,
    {
        if self >= (0_i8).into() {
            self.try_into().unwrap()
        } else {
            (-self).try_into().unwrap()
        }
    }
}

impl IAbs for i32 {
    type Output = u32;
}

impl IAbs for i8 {
    type Output = u8;
}

impl IAbs for i16 {
    type Output = u16;
}

impl IAbs for i64 {
    type Output = u64;
}
