use rand::Rng;
use std::cmp::Ordering;
use std::io;

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
}

fn double(a: &mut [i32; 10]) {
    for n in 0..10 {
        a[n] *= 2;
    }
}
