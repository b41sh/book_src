use std::fs::DirBuilder;
use std::fs::create_dir;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::Write;


pub fn dir_test() {
    println!("{}", "-----------dir_test--------------------");

    let mut builder = DirBuilder::new();
    // creating all parent directories
    builder.recursive(true);

    let path = "/tmp/dir/subdir";
    let _dir = builder.create(path);
    let _dir = match _dir {
        Ok(_dir) => _dir,
        Err(error) => {
            println!("create dir1 err {}", error);
        },
    };

    let path = "/tmp/dir2";
    let _dir = create_dir(path);
    let _dir = match _dir {
        Ok(_dir) => _dir,
        Err(error) => {
            println!("create dir2 err {}", error);
        },
    };

    // 递归创建目录
    let path = "/tmp/dir3/subdir3";
    let _dir = create_dir_all(path);
    let _dir = match _dir {
        Ok(_dir) => _dir,
        Err(error) => {
            println!("create dir3 err {}", error);
        },
    };
}

pub fn file_test() {
    println!("{}", "-----------file_test--------------------");

    let f = File::create("/tmp/foo");
    let mut f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("create file err {}", error)
        },
    };
    f.write_all(b"Hello").unwrap();

    let metadata = f.metadata().unwrap();
    println!("metadata = {:#?}", metadata);

    let file_type = metadata.file_type();
    assert_eq!(file_type.is_dir(), false);
    assert_eq!(file_type.is_file(), true);
    assert_eq!(file_type.is_symlink(), false);

}
