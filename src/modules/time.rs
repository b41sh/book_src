use std::time::Duration;
use std::time::Instant;
use std::time::SystemTime;
use std::thread::sleep;

pub fn time_test() {
    println!("{}", "-----------time_test--------------------");

    let d1 = Duration::new(5, 10);
    println!("d1 = {:#?}", d1);
    println!("d1.secs = {:#?}", d1.as_secs());
    println!("d1.micros = {:#?}", d1.subsec_micros());
    println!("d1.nanos = {:#?}", d1.subsec_nanos());

    let d2 = Duration::from_micros(1_000_002);
    println!("d2 = {:#?}", d2);
    println!("d2.secs = {:#?}", d2.as_secs());
    println!("d2.micros = {:#?}", d2.subsec_micros());
    println!("d2.nanos = {:#?}", d2.subsec_nanos());

    let d3 = Duration::from_millis(2569);
    println!("d3 = {:#?}", d3);
    println!("d3.secs = {:#?}", d3.as_secs());
    println!("d3.micros = {:#?}", d3.subsec_micros());
    println!("d3.nanos = {:#?}", d3.subsec_nanos());

    let i1 = Instant::now();
    println!("i1 = {:#?}", i1);

    i1.elapsed();
    let three_secs = Duration::from_secs(3);
    sleep(three_secs);
    println!("i1 = {:#?}", i1.elapsed());

    let now = SystemTime::now();
    println!("now = {:#?}", now);

}
