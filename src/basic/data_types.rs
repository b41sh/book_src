pub fn scalar_test() {
    println!("{}", "-----------scalar_test--------------------");

    // 布尔类型
    let b1 = true;
    let b2: bool = false;
    println!("b1 = {}", b1);
    println!("b2 = {}", b2);
    println!("b1 as i32 = {}", b1 as i32);
    println!("b2 as i32 = {}", b2 as i32);

    // 整型
    // u8、u16、u32、u64、u128、usize
    // i8、i16、i32、i64、i128、isize
    let n1: u8 = 255;
    let n2 = 100u16;
    let n3 = 0o200;  // 八进制
    let n4 = 0x300;  // 十六进制
    let n5 = 0b1001; // 二进制
    let n6 = b'*';   // 字面量 42u8
    println!("n1 = {}", n1);
    println!("n2 = {}", n2);
    println!("n3 = {}", n3);
    println!("n4 = {}", n4);
    println!("n5 = {}", n5);
    println!("n6 = {}", n6);
    println!("u128MIN = {}", std::u128::MIN);
    println!("u128MAX = {}", std::u128::MAX);
    println!("i128MIN = {}", std::i128::MIN);
    println!("i128MAX = {}", std::i128::MAX);

    let sum = 1 + 2;
    let sub = 10 - 2;
    let mul = 3 * 2;
    let div = 15 / 2;
    let rem = 15 % 2;
    println!("sum = {}", sum);
    println!("sub = {}", sub);
    println!("mul = {}", mul);
    println!("div = {}", div);
    println!("rem = {}", rem);


    // 浮点型
    // f32、f64
    let f1 = 1.111f32;
    let f2: f64 = 2.111;
    println!("f1 = {}", f1);
    println!("f2 = {}", f2);

    // 字符类型
    let c1 = 'c';
    let c2 = '的';
    println!("c1 = {}", c1);
    println!("c2 = {}", c2);

    //
    //
    // 范围类型
    //
    // 切片类型
    //
    // str字符串类型
    //
    // 原生指针
    //
    // never 类型
    //
    //
}

pub fn compound_test() {

    println!("{}", "-----------compound_types_test--------------------");

    // 元组（tuple）
    let t1: (i32, f64, u8) = (500, 6.4, 1);
    println!("t1.0 = {}", t1.0);
    println!("t1.1 = {}", t1.1);
    println!("t1.2 = {}", t1.2);
    let t2 = (500, 6.4, 1);
    //println!("t2= {}", t1);

    // 解构（destructuring）
    let (x, y, z) = t2;
    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);

    // 数组（array）
    // 元素类型相同、固定长度，栈上分配
    let a1: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a1[0] = {}", a1[0]);
    println!("a1[1] = {}", a1[1]);
    println!("a1[2] = {}", a1[2]);
    println!("a1[3] = {}", a1[3]);
    println!("a1[4] = {}", a1[4]);
    let a2 = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("a2[0] = {}", a2[0]);
    println!("a2[1] = {}", a2[1]);


}
