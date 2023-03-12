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
