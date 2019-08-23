
// 结构体和枚举可以定义泛型
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// 方法中的泛型
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

pub fn generic_test() {
    println!("{}", "-----------generic_test--------------------");

    let list = vec![2,3,4,5];
    let result = largest(&list);

    println!("result = {}", result);

    let p1 = Point{x: 5, y:10};
    let p2 = Point{x: 5.1, y:10.4};
    println!("p1 = {:#?}", p1);
    println!("p2 = {:#?}", p2);
    println!("p1.x = {:#?}", p1.x());
    println!("p2.x = {:#?}", p2.x());

}

// 函数可以定义泛型
// error[E0369]: binary operation `>` cannot be applied to type `T`
//
// error[E0508]: cannot move out of type `[T]`, a non-copy slice
// move occurs because `list[_]` has type `T`, which does not implement the `Copy` trait
// help: consider borrowing here: `&list[0]`
//fn largest<T>(list: &[T]) -> T {
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

