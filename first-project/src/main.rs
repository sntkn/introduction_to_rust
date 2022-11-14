fn main() {
    println!("Hello, world!");
    let x = 100; // immutable
    //x = 200;
    let mut y = 50; // mutable
    println!("{} {}", x, y);
    y = 100;
    println!("{} {}", x, y);

    let str_slice = "world"; // immutable(&str)
    let _string = String::from(str_slice); // 可変文字列を扱うには String型
    let _string_format = format!("Hello, {}", str_slice);

    // 固定長配列
    let mut array = [1,2,3];
    array[0] = 10;
    //array.push(10);

    // 可変長配列
    let mut vec = vec![1,2,3];
    vec[0] = 10;
    vec.push(10);

    // tuple
    let t = (120, "string");
    let t0 = t.0;
    let t1 = t.1;
    println!("{},{}", t0, t1);

    let x = 100;
    let y = 50;
    if x == y {
        println!("same value");
    } else {
        println!("different value");
    }
    // 三項演算子みたいなやつ
    let _z = if x != y { 500 } else { 300 };

    // loop
    for i in 0..3 {
        println!("in for-loop {}", i);
    }

    let mut count = 0;
    while count < 3 {
        println!("while-loop {}", count);
        count += 1;
    }

    loop {
        count -= 1;
        println!("loop {}", count);
        if count == 0 {
            break;
        }
    }

    // switch
    let i = 5;
    match i {
        0 => println!("zero"),
        1 => println!("one"),
        2|3 => println!("two or three"),
        4..=10 => println!("four or ten"),
        _ => println!("other"),
    }

    let is_zero_str = match i {
        0 => "zero",
        _ => "not zero",
    };
    println!("{}", is_zero_str);

    // use function
    let added = add(1, 2);
    println!("{}", added);


    // closure, 関数の中で外の変数を使う, 関数の書き方が違う
    let z = 20;
    let add_z = |x: i32, y: i32| x + y + z;
    println!("{}", add_z(10, 20));

    let banana = Fruit {
        name: String::from("Banana"),
    };
    println!("{}", banana.get_name());

    let rect = Rectangle(10, 20);
    println!("{}", rect.calc_area());

    let _unit = Unit;

    let color = Color::Red;
    println!("{}", color.color_code());
    let color: Color = Color::Custom(10, 122, 255);
    println!("{}", color.color_code());

    // 所有権
    let x = String::from("hello");
    let y = x; // ムーブ
    //println!("{}{}", x, y); // 移動済み value borrowed here after move
    println!("{}", y);

    {
        let z = String::from("world");
        {
            let w = &z; // 借用（＆つける？）
            // let s = z;
            println!("w:{}", w);
        } // w の解放
        println!("z:{}", z);
    }// z の解放

    let hello = "Hello";
    {
        let greet = Greet { word: hello};
        greet.say();
    } // greet 解放
    println!("{}", hello);

    let person: Person = Person(String::from("Tom"));
    person.greet();

}

// function
fn add(x: i32, y: i32) -> i32 {
    x + y
}

// 構造体

// name という名前付きの構造体
struct Fruit {
    name: String,
}

// 構造体を使うやつ（同じ名前じゃないといけない？）
impl Fruit {
    fn get_name(&self) -> &str {
        &self.name
    }
}

// 名前なしの構造体
struct Rectangle(i32, i32);

impl Rectangle {
    fn calc_area(&self) -> i32 {
        &self.0 * &self.1
    }
}

// 値を持たない構造体
struct Unit;

// 列挙型

enum Color {
    Red,
//    Green, // variants `Red` and `Green` are never constructed
//    Blue,
    Custom(u8, u8, u8),
}

impl Color {
    fn color_code(&self) -> String {
        match &self {
            Color::Red => String::from("#ff0000"),
//            Color::Green => String::from("#00ff00"),
//            Color::Blue => String::from("00ffff"),
            Color::Custom(r,g ,b ) => {
                format!("#{:02x}{:02x}{:02x}", r, g, b)
            }
        }
    }
}

// Option型, Result型
//enum Option<T> {
//    None,
//    Some(T),
//}
//
//enum Result<T, E> {
//    Ok(T),
//    Err(E),
//}

struct Greet<'a> { // 'a はライフタイムパラメータ
    word: &'a str,
}

impl<'a> Greet<'a> {
    fn say(&self) {
        println!("{}", self.word);
    }
}

// trait
trait Greeter {
    fn greet(&self);
}

struct Person(String);

impl Greeter for Person {
    fn greet(&self) {
        println!("Hello, I am {}!", self.0);
    }
}

#[derive(Debug)]
struct Hours(u32);
