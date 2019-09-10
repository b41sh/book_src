use std::env;
use learn::modules::result1;
use learn::modules::result2;
use learn::modules::result3;
use learn::modules::result4;

fn main() {
    println!("{}", "-----------result_test--------------------");
    let args:Vec<String> = env::args().collect();
    println!("{:?}", args);
    let filename = &args[1];
    println!("{:?}", filename);

    //result1::run(filename.to_string());
    result1::run(filename);

    match result2::run(filename) {
        Ok(n) => { println!("sum = {:?}", n); }
        Err(err) => { println!("sum err = {:?}", err); }
    }

    match result3::run(filename) {
        Ok(n) => { println!("sum = {:?}", n); }
        Err(err) => { println!("sum err = {:?}", err); }
    }

    match result4::run(filename) {
        Ok(n) => { println!("sum = {:?}", n); }
        Err(err) => { println!("sum err = {:?}", err); }
    }
}
