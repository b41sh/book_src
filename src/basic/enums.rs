// 枚举（enumerations），也被称作 enums
// 枚举允许你通过列举可能的值来定义一个类型
// Rust 的枚举与 F#、OCaml 和 Haskell 这样的函数式编程语言中的 代数数据类型（algebraic data types）最为相似

#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}

// 枚举可以包含多种不同类型的结构
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

pub fn enum_test() {
    println!("{}", "-----------enum_test--------------------");

    let four = IpAddrKind::V4(String::from("127.0.0.1"));
    let six = IpAddrKind::V6(String::from("0::0"));

    //error[E0277]: `basic::enums::IpAddrKind` doesn't implement `std::fmt::Display`
    //= help: the trait `std::fmt::Display` is not implemented for `basic::enums::IpAddrKind`
    //= note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
    //= note: required by `std::fmt::Display::fmt`
    println!("four = {:?}", four);
    println!("six = {:#?}", six);

    let mquit = Message::Quit;
    let mmove = Message::Move{x:11,y:22};
    let mwrite = Message::Write(String::from("message write"));
    let mchange = Message::ChangeColor(1,2,3);

    println!("quit = {:#?}", mquit);
    println!("move = {:#?}", mmove);
    println!("write = {:#?}", mwrite);
    println!("change = {:#?}", mchange);

    // option 可以包含空值
    let snum = Some(5);
    let sstr = Some("a string");
    let anum: Option<i32> = None;
    println!("some number = {:#?}", snum);
    println!("some string = {:#?}", sstr);
    println!("absent_number = {:#?}", anum);

    // match 控制流运算符
    let c = Coin::Penny;
    let v = value_in_cents(c);
    println!("v = {:#?}", v);

    let c = Coin::Nickel;
    let v = value_in_cents(c);
    println!("v = {:#?}", v);

    let c = Coin::Dime;
    let v = value_in_cents(c);
    println!("v = {:#?}", v);

    let c = Coin::Quarter;
    let v = value_in_cents(c);
    println!("v = {:#?}", v);

}

// match 是极为强大的控制流运算符
// 它允许我们将一个值与一系列的模式相比较并根据相匹配的模式执行相应代码
// 模式可由字面值、变量、通配符和许多其他内容构成
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

