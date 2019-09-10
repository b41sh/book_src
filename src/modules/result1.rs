use std::fs::File;
use std::io::prelude::*;

//pub fn run(filename: String) {
pub fn run(filename: &str) {
    let mut f = File::open(filename).unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    let mut sum = 0;
    for c in contents.lines() {
        let n = c.parse::<i32>().unwrap();
        sum += n;
    }
    println!("sum = {:?}", sum);
}
