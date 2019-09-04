use std::path::Path;
use std::path::PathBuf;

pub fn path_test() {
    println!("{}", "-----------path_test--------------------");

    let path = Path::new("/usr/include/ffi/ffi.h");
    let parent = path.parent();
    let file_stem = path.file_stem();
    let extension = path.extension();

    println!("path = {}", path.display());
    println!("parent = {:#?}", parent);
    println!("file_stem = {:#?}", file_stem);
    println!("extension = {:#?}", extension);

    for ancestor in path.ancestors() {
        println!("ancestor = {:#?}", ancestor);
        println!("ancestor = {}", ancestor.display());
    }
    for component in path.components() {
        println!("component = {:#?}", component);
    }

    let mut path_buf = PathBuf::new();
    path_buf.push("/usr");
    path_buf.push("local");
    path_buf.push("bin");
    println!("path_buf = {:#?}", path_buf);

}
