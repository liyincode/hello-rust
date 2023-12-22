use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let a: i32 = 1;

    let hello = String::from("🤑");

    // let byte_escape = "I'm saying hello \x7f dd";

    let byte_escape = r"\u{0064}";

    let bytestring: &[u8; 21] = b"this is a byte string";
    println!("A byte string: {:?}", bytestring);

    println!("{}", byte_escape);

    let arra: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", arra[2]);

    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    println!("{}", v[2]);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{}", v[2]);

    struct User {
        active: bool,
        username: String,
        email: String,
        age: u32,
    }

    let user1 = User {
        email: String::from("x@gmail.com"),
        username: String::from("x"),
        active: true,
        age: 20,
    };

    println!("{}", user1.age);

    enum IpAddrKind {
        V4,
        V6,
    }

    let four = IpAddrKind::V4;

    let number1 = 6;
    if number1 % 4 == 0 {
        println!("number is divisible by 4");
    } else if number1 % 3 == 0 {
        println!("number is divisible by 3");
    } else if number1 % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let x1 = 1;
    let y1 = if x1 == 0 { 100 } else { 200};
    println!("{}", y1);

    loop_test();
    while_test();
    for_test();
    closure_test();
    shadowing_test();
    variable_type_test();
    refer_test();
    string_test();
    struct_test();
    enum_test();
    type_test();
}

fn loop_test() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn while_test() {
    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_test() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("{}", element);
    }

    for number in 1..4 {
        println!("{}", number);
    }

    for number in (1..4).rev() {
        println!("{}", number);
    }

    for number in 1..=4 {
        println!("{}", number);
    }

    for ch in 'a'..='z' {
        println!("{}", ch);
    }
}

fn closure_test() {
    fn add_one_v1 (x: u32) -> u32 { x + 1 }

    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x: u32| { x + 1};
    let add_one_v3 = |x: u32| x + 1;

    let a = 1;
    let add_v1 = |x: u32| -> u32 { x + a };
    let result1 = add_v1(100);
    println!("{}", result1);

}

fn add_one_v1 (x: u32) -> u32 { x + 1 }
#[cfg(test)]
mod tests {
    use crate::add_one_v1;

    #[test]
    fn it_works() {
        let result = add_one_v1(2);
        assert_eq!(3, result);
    }
}

fn shadowing_test() {
    println!("shadowing_test------");
    let x = 5;
    println!("{}", x);
    let x = 6;
    println!("{}", x);
}

fn foo(s: String) {
    println!("{}", s);
    s;
}
fn variable_type_test() {
    println!("variable_type_test------");
    // let a: u8 = 323232;
    let a:i128 = 2343242342343242343215125215252521;
    // println!("{}", a);

    // 所有权
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1);
    println!("{}", s2);

    let s3 = String::from("hello3");
    let s3 = foo(s3);
    println!("{:?}", s3);

    let s3 = "hello3".to_string();
    for i in 1..10 {
        let temp_s = s3.clone();
        println!("{}", temp_s);
    }

    struct Point {
        x: i32,
        y: i32,
    }
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1;
    println!("{}", p2.x);

}

// 引用
fn refer_test() {
    // let mut a = 10u32;
    // let b = &a;
    // let c = &&&&&a;
    // let d = &b;
    // let e = b;
    //
    // println!("a = {}", a);
    // println!("b = {}", b);
    // println!("c = {}", c);
    // println!("d = {}", d);
    // println!("e = {}", e);
    // println!("&a = {:p}", a);

    /*
    如果一个值有可变引用，那这个值在这个引用没有销毁前是不能被访问的
     */
    // let mut a = 10u32;
    // let b = &mut a;
    // *b = 20;
    //
    // // println!("a = {}", a); // 这里会报错
    // println!("b = {}", b);

    /*
    // 当有一个可变引用存在时，并且还没有被销毁时，这个值是不能被引用的
    let mut a = 10u32;
    let b = &mut a;
    *b = 20;
    let c = &a; // 报错

    println!("b = {}", b);
     */

    /*[
    let mut a = 10u32;
    let r1 = &mut a;
    let r2 = r1;

    println!("r1 = {}", r1); // 报错, r1 其实已经被销毁了
     */

    /*
    // *c 指的是指向 a1 的指针 b，而不是 a1 这个值，所以 *c 的类型是 &mut u32（指针类型），并不是 u32 类型
    // 所以不能直接赋值给 *c
    let mut a1 = 10u32;
    let mut b = &mut a1;
    *b = 20;

    let c = &mut b;
    *c = 30; // 报错

    println!("c = {}", c);

     */
}

fn string_test() {
    let s1: &'static str = "I am a superman.";
    let mut s2: String = s1.to_string();
    // let s3: &String = &s2;
    // let s4: &str = &s2[..];
    // let s5: &str = &s2[..6];


    // {
    //     let s6 = &mut s2;
    // }
    //
    // println!("{}", s2);
    // println!("{}", s6);

    fn foo(s: &str) {

    }

    let s = String::from("hello");
    foo(&s);
    let ss = "world";
    foo(ss);

    let s11 = String::from("hello");
    let char_vec: Vec<char> = s11.chars().collect();

    let a = "10".parse::<u32>();
    let aa: u32 = "10".parse().unwrap();
    println!("{}", aa);
}

fn struct_test() {
    #[derive(Debug)]
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let user1 = User {
        email: String::from("aa@aa.com"),
        username: String::from("aa"),
        active: true,
        sign_in_count: 1,
    };
    println!("{:?}", user1);

    let user2 = User {
        email: String::from("bb@bb.com"),
        ..user1
    };

    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
    println!("{}", black.0);

    struct ArticleModule;
    let module = ArticleModule;

    #[derive(Default, Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(self) -> u32 {
            self.width * self.height
        }
    }

    let rect = Rectangle {
        width: 10,
        height: 20,
    };
    println!("{}", rect.area());

    let rect2: Rectangle = Default::default();
    println!("{}", rect2.area());
}

#[derive(Debug)]
enum Shape1 {
    Rectangle,
    Triangle,
    Circle,
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

enum Shape {
    Rectangle(Rectangle),
    Triangle((u32, u32), (u32, u32), (u32, u32)),
    Circle { origin: (u32, u32), radius: u32 },
}
fn enum_test() {

    // fn test() {
    //     let shape_a = Shape::Rectangle;
    //     match shape_a {
    //         Shape::Rectangle => {
    //             println!("Rectangle");
    //         },
    //         Shape::Triangle => {
    //             println!("Triangle");
    //         },
    //         Shape::Circle => {
    //             println!("Circle");
    //         },
    //     }
    // }
    // test();

    let a_rec = Rectangle {
        width: 10,
        height: 20,
    };

    let shape_a = Shape::Rectangle(a_rec);

    match shape_a {
        Shape::Rectangle(rect) => {
            println!("Rectangle: {:?}", rect);
        },
        Shape::Triangle(point_a, point_b, point_c) => {
            println!("Triangle: {:?}, {:?}, {:?}", point_a, point_b, point_c);
        },
        Shape::Circle { origin, radius } => {
            println!("Circle: {:?}, {:?}", origin, radius);
        },
    }

}

struct Point<T, U> {
    s: T,
    y: U,
}
fn type_test() {
    let a = 1.0f32;
    let b = 10 as f32;
    let c = a * b;

    let integer = Point { s: 5, y: 10 };
    let float = Point { s: 1.0, y: 4.0 };
    let other = Point { s: 5, y: 4.0 };

    let integer1 = Point::<u32, u32> { s: 5, y: 10 };
}