// Rust 的 闭包（closures）是可以保存进变量或作为参数传递给其他函数的匿名函数。
// 可以在一个地方创建闭包，然后在不同的上下文中执行闭包运算。
// 不同于函数，闭包允许捕获调用者作用域中的值。
use std::thread;
use std::time::Duration;

pub fn closure_test() {
    println!("{}", "-----------closure_test--------------------");

    // 我们希望能够在程序的一个位置指定某些代码，
    // 并只在程序的某处实际需要结果的时候执行这些代码。这正是闭包的用武之地！

    // 闭包定义是 expensive_closure 赋值的 = 之后的部分。
    // 闭包的定义以一对竖线（|）开始，在竖线中指定闭包的参数；
    // 之所以选择这个语法是因为它与 Smalltalk 和 Ruby 的闭包定义类似。
    // 这个闭包有一个参数 num；如果有多于一个参数，可以使用逗号分隔，比如 |param1, param2|。

    // 参数之后是存放闭包体的大括号 —— 如果闭包体只有一行则大括号是可以省略的。
    // 大括号之后闭包的结尾，需要用于 let 语句的分号。
    // 因为闭包体的最后一行没有分号（正如函数体一样），所以其返回了值 num 。

    // let 语句意味着 expensive_closure 包含一个匿名函数的定义，不是调用匿名函数的返回值。

    // 闭包不要求像 fn 函数那样在参数和返回值上注明类型。
    //let expensive_closure = |num: u32| -> u32 {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let intensity = 24;
    let random_number = 4;

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_closure(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            );
        }
    }

    // 闭包不同的写法
    // fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    // let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = |x|             { x + 1 };
    // let add_one_v4 = |x|               x + 1  ;

}
