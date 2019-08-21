// 内存管理的三种方式
// 1. 程序员分配和释放
// 2. 垃圾回收(gc)
// 3. 所有权(ownership)
//
// 所有权规则
// 1. Rust 中的每一个值都有一个被称为其 所有者（owner）的变量。
// 2. 值有且只有一个所有者。
// 3. 当所有者（变量）离开作用域，这个值将被丢弃。
//

// 作用域（scope）是一个项（item）在程序中有效的范围。
pub fn scope_test() {
    println!("{}", "-----------scope_test--------------------");
    {
        {
            {
                // 变量从声明的点开始直到当前 作用域 结束时都是有效的。
                // 变量离开作用域时会被移出栈
                let s = "hello";
                println!("s = {}", s);
            }
            // 作用域外变量失效，编译错误
            // error[E0425]: cannot find value `s` in this scope
            // println!("s = {}", s);
            // 自动调用drop函数释放内存
            // 类似C++的资源获取即初始化（Resource Acquisition Is Initialization (RAII)）
        }
    }
}

pub fn ownership_test() {
    println!("{}", "-----------ownership_test--------------------");

    // 栈上的数据，直接复制
    let i1 = 10;
    let i2 = i1;
    println!("i1 = {} i2 = {}", i1, i2);

    // 实现了 copy trait 的类型
    // 1. 所有整数类型，比如 u32。
    // 2. 布尔类型，bool，它的值是 true 和 false。
    // 3. 所有浮点数类型，比如 f64。
    // 4. 字符类型，char。
    // 5. 元组，当且仅当其包含的类型也都是 Copy 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是。

    // 堆上的数据不能直接复制，对性能影响较大
    // 赋值的时候回发生所有权转移，s1释放，不再有效
    // Rust 永远也不会自动创建数据的 “深拷贝”
    let s1 = String::from("hello");
    let s2 = s1;
    // error[E0382]: borrow of moved value: `s1`
    // println!("s1 = {} s2 = {}", s1, s2);
    println!("s2 = {}", s2);

    // 如果确实想复制堆上的数据，可以使用 clone 方法
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}", s3, s4);

}

pub fn ownership_fn_test() {
    println!("{}", "-----------ownership_func_test--------------------");

    let i1 = 10;
    let s1 = String::from("hello");
    // i1 copy 到函数中，后续可以继续使用
    // s1 move 到函数中，所有权已转移，后续不能使用
    fn1(i1, s1);

    println!("out fn i1 = {}", i1);
    // error[E0382]: borrow of moved value: `s1`
    //println!("out fn s1 = {}", s1);
}

fn fn1(i1: i32, s1: String) {

    println!("in fn i1 = {}", i1);
    println!("in fn s1 = {}", s1);

}

pub fn references_test() {
    println!("{}", "-----------references_test--------------------");

    let s1 = String::from("hello");

    // & 符号就是 引用，它们允许你使用值但不获取其所有权
    // s 指向 s1，但是并不拥有它
    let len = cal_len(&s1);

    println!("s1 = {}, len = {}", s1, len);

    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("s2 = {}", s2)

}

// 获取引用作为函数参数称为 借用（borrowing）
fn cal_len(s: &String) -> usize {
    // error[E0596]: cannot borrow `*s` as mutable, as it is behind a `&` reference
    // ^ `s` is a `&` reference, so the data it refers to cannot be borrowed as mutable
    // s.push_str(", world");
    s.len()
}

// 添加mut 变为可变引用
fn change(s: &mut String) {
    s.push_str(", world");
    // 特定作用域中的特定数据有且只有一个可变引用
    // 编译时避免数据竞争（data race）
    // 1. 两个或更多指针同时访问同一数据。
    // 2. 至少有一个指针被用来写入数据。
    // 3. 没有同步数据访问的机制。

    // 引用规则
    // 1. 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
    // 2. 引用必须总是有效。
}

// 字符串 slice（string slice）是 String 中一部分值的引用
pub fn slice_test() {
    println!("{}", "-----------slice_test--------------------");

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("h = {}, w = {}", hello, world);

    let hello1 = &s[0..=4];
    let world1 = &s[6..=10];

    println!("h = {}, w = {}", hello1, world1);

    // 字符串字面值就是slice
    let s = "Hello world!";
    println!("s = {}", s);

    let s1 = String::from("Hello world");
    let w = first_word(&s1);
    println!("w = {}", w);

}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

