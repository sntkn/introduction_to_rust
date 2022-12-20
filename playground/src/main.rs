fn main() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    let rect = Rectangle {
        width: 50,
        height: 50,
    };

    println!("{}", rect.area());

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.can_hold(&rect)
    );

    let mut arr = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    double(&mut arr);
    println!("{:?}", arr);

    println!("{}", f(12i16, 13i16));

    let mut v = vec![11, 22, 33];
    for _ in 0..5 {
        let item: Option<i32> = v.pop();
        let i = match item {
            Some(number) => number,
            None => -1,
        };
        print!("{}, ", i);
        if item.is_some() {
            print!("#{}, ", item.unwrap());
        } else if item.is_none() {
            print!("#-1, ")
        }
    }
    println!("");

    // sort desc
    let mut arr = [1, 2, 3, 34, 5, 6, 78];
    arr.sort_by(|a, b| b.cmp(a));
    // arr.sort_by(|a, b| (&-a).cmp(&-b));
    println!("{:?}", arr);

    // calling closure
    let factor = 2;
    let multiply = |a| a * factor;
    let multiply_ref = &multiply;
    println!(
        "{} {} {} {} {}",
        (*multiply_ref)(13),           // デリファレンス
        multiply_ref(13),              // デリファレンスしない
        (|a| a * factor)(13),          // クロージャ
        (|a: i32| a * factor)(13),     // クロージャ
        |a| -> i32 { a * factor }(13), // クロージャ
    );

    print_nth_char("€fg", 1);
    print_codes("€fg");
    let v = vec![10, 20, 30];
    for item in v.into_iter() {
        println!("{} ", item+1);
    }
    //print!("{:?}", v);
    mod routines {
        pub fn f() -> u32 { g() }
        fn g() -> u32 { 123 }
        pub mod m {
            pub fn f() {
                println!("routines::m::f");
                println!("{}", super::f()); //
            }
            pub mod mm {
                pub fn f() {
                    println!("routines::m::mm::f");
                    println!("{}", super::super::f());
                }
            }
        }
    }
    println!("{}", routines::f());
    routines::m::f();
    routines::m::mm::f();
}

fn double(a: &mut [i32; 10]) {
    for n in 0..10 {
        a[n] *= 2;
    }
}

fn f<T>(a: T, _b: T) -> T {
    a
}

fn print_nth_char(s: &str, mut n: u32) {
    let mut iter: std::str::Chars = s.chars();
    loop {
        let item: Option<char> = iter.next();
        match item {
            Some(c) => {
                if n == 0 {
                    println!("{}", c);
                    break;
                }
            }
            None => {
                break;
            }
        }
        n -= 1;
    }
}

fn print_codes(s: &str) {
    for c in s.chars() {
        println!("{}: {}", c, c as u32);
    }
    // let mut iter = s.chars();
    // loop {
    //     match iter.next() {
    //         Some(c) => println!("{}: {}", c, c as u32),
    //         None => break,
    //     }
    // }
}
