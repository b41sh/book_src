use std::vec::Vec;

pub fn vec_test() {
    println!("{}", "-----------vec_test--------------------");

    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    assert_eq!(vec.len(), 2);
    assert_eq!(vec[0], 1);

    assert_eq!(vec.pop(), Some(2));
    assert_eq!(vec.len(), 1);

    vec[0] = 7;
    assert_eq!(vec[0], 7);

    vec.extend([1, 2, 3].iter().cloned());

    for x in &vec {
        println!("{}", x);
    }
    assert_eq!(vec, [7, 1, 2, 3]);

    // 也可以使用 vec! 宏初始化 
    let mut vec2 = vec![1, 2, 3];
    vec2.push(4);
    assert_eq!(vec2, [1, 2, 3, 4]);

    // 按个数初始化
    let vec3 = vec![0; 5];
    assert_eq!(vec3, [0, 0, 0, 0, 0]);

    let mut vec4 = Vec::with_capacity(5);
    vec4.resize(10, 1);
    for x in vec4 {
        println!("{}", x);
    }

    // 用 vec 做 stack
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        // 3 2 1
        println!("{}", top);
    }

    // 支持按 Index 访问，Vec 实现了 Index trait
    let vec5 = vec![1, 2, 3];
    println!("{}", vec5[2]);
    // panic
    // println!("{}", vec5[3]);


}
