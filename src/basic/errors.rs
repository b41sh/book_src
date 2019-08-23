// Rust 将错误组合成两个主要类别：可恢复错误（recoverable）和 不可恢复错误（unrecoverable）。
// 可恢复错误通常代表向用户报告错误和重试操作是合理的情况，比如未找到文件。
// 不可恢复错误通常是 bug 的同义词，比如尝试访问超过数组结尾的位置。
// Rust 并没有异常。对于可恢复错误有 Result<T, E> 值，以及 panic!，它在遇到不可恢复错误时停止程序执行。

// 当出现 panic 时，程序默认会开始 展开（unwinding），这意味着 Rust 会回溯栈并清理它遇到的每一个函数的数据，不过这个回溯并清理的过程有很多工作。
// 另一种选择是直接 终止（abort），这会不清理数据就退出程序。
// 通过在 Cargo.toml 的 [profile] 部分增加 panic = 'abort'，可以由展开切换为终止。

use std::fs::File;
use std::io;
use std::io::Read;
use std::io::ErrorKind;

pub fn error_test() {
    println!("{}", "-----------error_test--------------------");

    // 不可恢复错误
    //panic!("crash and burn");

    //let v = vec![1, 2, 3];

    // 溢出 panic
    // thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', /rustc/0beb2ba16a08dfa01569b5f4644da315dc4c806c/src/libcore/slice/mod.rs:2681:10
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
    //v[99];
    
    // 可恢复错误
    let f = File::open("hello.txt");

    // 使用 match 处理 Result 成员
    let f = match f {
        Ok(file) => file,
        Err(err) => {
            panic!("open file failed: {:#?}", err);
        }
    };
    println!("file is = {:#?}", f);

    let f = File::open("hello.txt");
    // 区分不同的错误类型
    let f = match f {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file2) => file2,
                Err(err2) => {
                    panic!("create file failed: {:?}", err2);
                }
            },
            err3 => {
                panic!("open file failed: {:?}", err3);
            }
        }
    };
    println!("file is = {:#?}", f);

    // 以闭包的方式处理，更简单。更老练的 Rustacean
    let f = File::open("hello.txt").map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("create file failed: {:#?}", error);
            });
        } else {
            panic!("open file failed: {:#?}", error);
        }
    });
    println!("file is = {:#?}", f);

    // 更简单的写法
    // 如果 Result 值是成员 Ok，unwrap 会返回 Ok 中的值。
    // 如果 Result 是成员 Err，unwrap 会为我们调用 panic!。
    let f = File::open("hello.txt").unwrap();
    println!("file is = {:#?}", f);

    // 等效的写法，指定panic的内容
    let f = File::open("hello.txt").expect("open file failed");
    println!("file is = {:#?}", f);

    // 传播（propagating）错误，让调用者知道这个错误并决定该如何处理。
    let s = read_file().unwrap();
    println!("file content is = {:#?}", s);

}

// 错误传播，留给上层处理
// ? 只能被用于返回 Result 的函数
fn read_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    // 更简单的写法
    //File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
