use std::vec::Vec;
use std::collections::VecDeque;

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
        println!("vec = {}", x);
    }
    assert_eq!(vec, [7, 1, 2, 3]);

    // 也可以使用 vec! 宏初始化 
    let mut vec2 = vec![1, 2, 3];
    vec2.push(4);
    assert_eq!(vec2, [1, 2, 3, 4]);

    // 按个数初始化
    let vec3 = vec![0; 5];
    assert_eq!(vec3, [0, 0, 0, 0, 0]);

    // use `with_capacity` when you know exactly how many elements will be inserted,
    // or at least have a reasonable upper-bound on that number.
    let mut vec4 = Vec::with_capacity(5);
    vec4.extend([1, 2, 3].iter().cloned());

    // If you believe that a collection will not soon contain any more
    // elements, or just really need the memory
    vec4.shrink_to_fit();
    println!("c=={}", vec4.capacity());
    assert!(vec4.capacity() >= 3);
    vec4.resize(10, 1);
    for x in vec4 {
        println!("vec4 = {}", x);
    }

    // 用 vec 做 stack
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        // 3 2 1
        println!("stack = {}", top);
    }

    // 支持按 Index 访问，Vec 实现了 Index trait
    let vec5 = vec![1, 2, 3];
    println!("vec5 = {}", vec5[2]);
    // panic
    // println!("{}", vec5[3]);

    // iter
    let vec7 = vec![1, 2, 3, 4];
    for x in vec7.iter() {
        println!("vec7 = {}", x);
    }

    let mut vec8 = vec![1, 2, 3, 4];
    for x in vec8.iter_mut() {
        *x += 1;
        println!("mut vec8 = {}", x);
    }

}

// A double-ended queue implemented with a growable ring buffer.
pub fn vecdeque_test() {
    println!("{}", "-----------vecDeque_test--------------------");

    let mut vec = VecDeque::new();
    vec.push_back(1);
    vec.push_back(2);
    vec.push_back(3);
    println!("vec = {:#?}", vec.get(1));
    println!("vec = {:#?}", vec.get(2));
    if let Some(elem) = vec.get_mut(1) {
        *elem = 7;
    }
    println!("vec = {:#?}", vec.get(1));
    vec.swap(0, 2);
    assert_eq!(vec, [3, 7, 1]);
    assert!(vec.capacity() >= 3);

}
