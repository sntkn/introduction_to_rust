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

    println!("{}",f(12i16,13i16));

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
}

fn double(a: &mut [i32; 10]) {
    for n in 0..10 {
        a[n] *= 2;
    }
}

fn f<T>(a: T, _b: T) -> T {
    a
}
