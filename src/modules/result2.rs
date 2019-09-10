use std::fs::File;
use std::io::prelude::*;

// 定义类型别名
type Result<I32> = std::result::Result<I32, Box<dyn std::error::Error>>;

pub fn run(filename: &str) -> Result<i32> {

    // 使用组合子方法重构
    // map map_err and_then
    File::open(filename).map_err(|err| err.into())
    .and_then(|mut f| {
        let mut contents = String::new();
        f.read_to_string(&mut contents)
        .map_err(|err| err.into())
        .map(|_|contents)
    })
    .and_then(|contents| {
        let mut sum = 0;
        for c in contents.lines() {
            match c.parse::<i32>() {
                Ok(n) => {sum += n},
                //Err(err) => { let _err: Box<dyn std::error::Error> = err.into(); },
                Err(err) => { return Err(From::from(err)); },
            }
        }
        Ok(sum)
    })
}
