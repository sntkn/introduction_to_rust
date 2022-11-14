fn main() {
    println!("{}", div(4, 2));
}

fn div(x: i32, y: i32) -> i32 {
    x / y
}

// test は別モジュールにするのが一般的
#[cfg(test)]
mod tests {
    use super::*;

    #[test] // テストのたびに宣言
    fn div_test() {
        assert_eq!(div(10, 3), 3);
    }

    #[test]
    #[should_panic]
    fn div_panic_test() {
        div(2, 0);
    }
}
