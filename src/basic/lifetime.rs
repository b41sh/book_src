// Rust 中的每一个引用都有其 生命期（lifetime），也就是引用保持有效的作用域。
// 大部分时候生命期是隐含并可以推断的，正如大部分时候类型也是可以推断的一样。
// 也有部分生命期不能推断的情况，需要注明关系。

// 生命期注解并不改变任何引用的生命期的长短。
// 与当函数签名中指定了泛型类型参数后就可以接受任何类型一样，
// 当指定了泛型生命期后函数也能接受任何生命期的引用。
// 生命期注解描述了多个引用生命周期相互的关系，而不影响其生命周期。

// 生命期注解有着一个不太常见的语法：生命周期参数名称必须以撇号（'）开头，其名称通常全是小写，
// 类似于泛型其名称非常短。'a 是大多数人默认使用的名称。
// 生命期参数注解位于引用的 & 之后，并有一个空格来将引用类型与生命期注解分隔开。

// 单个的生命期注解本身没有多少意义，因为生命期注解告诉 Rust 多个引用的泛型生命期参数如何相互联系的。

pub fn lifetime_test() {
    println!("{}", "-----------lifetime_test--------------------");

    let r;
    {
        let x = 5;
        // 生命期避免了悬垂引用
        // Rust 编译器有一个 借用检查器（borrow checker），它比较作用域来确保所有的借用都是有效的。
        // error[E0597]: `x` does not live long enough
        r = &x;
        println!("r = {}", r);
    }
    //println!("r = {}", r);

    let x = String::from("abcd");
    let y = "abc";
    let z = longest(&x.as_str(), y);
    println!("z = {}", z);

    let s = "hello world";
    let w = first_word(s);
    println!("first word = {}", w);

    //let xx = String::from("aaa");
    let xx = "aaa";
    //{
        let yy = xx;
        println!("yy = {}", yy);
    //}
    println!("xx = {}", xx);

}

// 需要指定参数的生命期，不知道哪个值会返回
// error[E0106]: missing lifetime specifier
//fn longest(x: &str, y: &str) -> &str {
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 生命期省略（Lifetime Elision）
// 避免冗余代码，编译器自动推断
// 编译器采用三条规则来判断引用何时不需要明确的注解。
// 第一条规则适用于输入生命周期，后两条规则适用于输出生命周期。
// 如果编译器检查完这三条规则后仍然存在没有计算出生命周期的引用，编译器将会停止并生成错误。
//
// 第一条规则是每一个是引用的参数都有它自己的生命周期参数。
// 换句话说就是，有一个引用参数的函数有一个生命周期参数：
// fn foo<'a>(x: &'a i32)，有两个引用参数的函数有两个不同的生命周期参数，
// fn foo<'a, 'b>(x: &'a i32, y: &'b i32)，依此类推。
//
// 第二条规则是如果只有一个输入生命周期参数，
// 那么它被赋予所有输出生命周期参数：fn foo<'a>(x: &'a i32) -> &'a i32。
//
// 第三条规则是如果方法有多个输入生命周期参数，
// 不过其中之一因为方法的缘故为 &self 或 &mut self，那么 self 的生命周期被赋给所有输出生命周期参数。
// 这使得方法更容易读写，因为只需更少的符号。
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
