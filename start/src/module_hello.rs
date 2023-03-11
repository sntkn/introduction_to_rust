// mod module_hello { で囲まなくてOK
// 呼び出す側は mod module_hello; で module_hello::print_hello; を定義
// module_hello::print_hello(); または crate::module_hello::print_hello(); で呼びだす
// crate:: は自身の中にあるモジュールの場合につける（絶対パス）
// use module_hello と書いても良い（記述を少なく）
// use crate::module_a::module_b; -> module_b::func_b(); で呼びだせる

pub fn print_hello() {
    println!("Hello");
}

// ファイル名と別の名前のモジュールの場合
// #[path = "module_abc.rs"]
// mod module_hello;
