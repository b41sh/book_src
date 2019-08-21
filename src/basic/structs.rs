
// 通过注解增加 Debug trait
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    // 方法语法（method syntax）
    fn display(&self) {
        println!("dispaly fn: username = {}, email = {}", self.username, self.email);
    }

    // 函数（associated functions）
    // 不以self作为参数
    fn build(username: String, email: String) -> User { 
        User {
            username,
            email,
            sign_in_count: 10,
            active: true,
        }
    }
}

pub fn struct_test() {
    println!("{}", "-----------struct_test--------------------");

    let user1 = User {
        username: String::from("test"),
        email: String::from("test@gmail.com"),
        sign_in_count: 10,
        active: true,
    };

    let username = String::from("test2");
    let email = String::from("test2@gmail.com");

    let user2 = build_user(username, email);

    let user3 = User {
        email: String::from("another@gmail.com"),
        username: String::from("another"),
        // 结构体更新语法（struct update syntax）
        ..user1
    };

    let username = String::from("test4");
    let email = String::from("test4@gmail.com");
    let user4 = User::build(username, email);

    // error[E0277]: `basic::structs::User` doesn't implement `std::fmt::Display`
    // println!("{}", user1);
    // :? 指示符告诉 println! 我们想要使用叫做 Debug 的输出格式
    println!("{:?}", user1);
    println!("{:?}", user2);
    println!("{:?}", user3);
    println!("{:?}", user4);
    // 自动引用和解引用（automatic referencing and dereferencing）
    user1.display();

    // 元组结构体（tuple structs）
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{}", black.0);
    println!("{}", origin.0);

    // 类单元结构体（unit-like structs） 没有任何字段
    struct Test {
    }
    let _test = Test{};
}

fn build_user(username: String, email: String) -> User {
    User {
        //username: username,
        //email: email,
        // 字段初始化简写语法（field init shorthand）
        username,
        email,
        sign_in_count: 10,
        active: true,
    }
}
