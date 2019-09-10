
pub fn result_test() {
    println!("{}", "-----------result_test--------------------");

    let version = parse_version(&[1,2,3,4]);
    match version {
        Ok(v) => println!("version is {:?}", v),
        Err(err) => println!("err is {:?}", err),
    }
}

#[derive(Debug)]
enum Version {
    Version1,
    Version2,
}

// 使用字符串作为err
fn parse_version(header: &[u8]) -> Result<Version, &str> {
    match header.get(0) {
        None => Err("invalid header"),
        Some(&1) => Ok(Version::Version1),
        Some(&2) => Ok(Version::Version2),
        Some(_) => Err("invalid version")
    }
}
