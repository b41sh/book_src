pub fn var_test() -> () {
    println!("{}", "-----------var_test--------------------");

    // 声明局部变量(declarations of local variables)
    // let name[: type] [= value];
    // 默认不可变，不能重复赋值
    // local variables are, by default, immutable
    let i1: i32 = 11;
    // error[E0384]: cannot assign twice to immutable variable `i1`
    //i1 = 12;
    println!("i1 = {}", i1);

    // 在变量名之前加 mut 来使其可变
    let mut i2 = 12;
    println!("i2 = {}", i2);
    i2 = 13;
    println!("i2 = {}", i2);

    // 变量与常量(constants)的区别
    // 1. 不允许对常量使用 mut。常量不光默认不能变，它总是不能变。
    // 2. 声明常量使用 const 关键字而不是 let，并且 必须 注明值的类型。
    // 3. 常量可以在任何作用域中声明，包括全局作用域，这在一个值需要被很多部分的代码用到时很有用。
    // 4. 常量只能被设置为常量表达式，而不能是函数调用的结果，或任何其他只能在运行时计算出的值。
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS = {}", MAX_POINTS);

    // 隐藏（Shadowing）
    // 后赋值的变量隐藏之前赋值的变量
    // 后面的变量是新的，只是用了相同的名字，避免无意义的重复命名
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("x = {}", x);
}
