use std::vec::Vec;
use std::collections::VecDeque;
use std::collections::LinkedList;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::collections::BTreeSet;

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

pub fn linkedlist_test() {
    println!("{}", "-----------LinkedList_test--------------------");

    let mut list1 = LinkedList::new();
    list1.push_back(1);
    list1.push_back(2);

    let mut list2 = LinkedList::new();
    list2.push_back(3);
    list2.push_back(4);

    list1.append(&mut list2);

    println!("list1 = {:#?}", list1);
    // list2 is empty
    println!("list2 = {:#?}", list2);

    // forward iterator
    let mut iter = list1.iter();
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&4));
    assert_eq!(iter.next(), None);

    assert!(!list1.is_empty());
    assert!(list2.is_empty());

    assert_eq!(list1.len(), 4);

    assert_eq!(list1.contains(&1), true);

    assert_eq!(list1.front(), Some(&1));
    assert_eq!(list1.back(), Some(&4));

    list1.clear();
    assert_eq!(list1.len(), 0);

    list1.push_front(0);
    println!("list1 = {:#?}", list1);
    assert_eq!(list1.pop_front(), Some(0));

    list1.push_back(5);
    println!("list1 = {:#?}", list1);
    assert_eq!(list1.pop_back(), Some(5));

    list1.push_front(1);
    list1.push_front(2);
    list1.push_front(3);

    let splitted = list1.split_off(2);
    println!("list1 = {:#?}", list1);
    println!("splitted = {:#?}", splitted);

}

pub fn hashmap_test() {
    println!("{}", "-----------HashMap_test--------------------");
    let mut map1 = HashMap::new();
    map1.insert(1, "val1".to_string());
    println!("map1 = {:#?}", map1);

    // 借用数据
    //error[E0308]: mismatched types
    // note: expected type `&{integer}` found type `{integer}`
    // help: consider borrowing here: `&1`
    //assert!(map1.contains_key(1));
    assert!(map1.contains_key(&1));
    assert!(!map1.contains_key(&2));
    assert!(!map1.is_empty());

    map1.insert(2, "val2".to_string());
    map1.insert(3, "val3".to_string());

    println!("capacity = {:#?}", map1.capacity());
    println!("len = {:#?}", map1.len());

    // 获取keys
    for key in map1.keys() {
        println!("key = {:#?}", key);
    }

    // 获取values
    for val in map1.values() {
        println!("val = {:#?}", val);
    }

    // 可变values
    for val in map1.values_mut() {
        *val = "abc".to_string();
        println!("val = {:#?}", val);
    }

    // 遍历
    for (key, val) in map1.iter() {
        println!("key: {} val: {}", key, val);
    }

    for (_, val) in map1.iter_mut() {
        *val = "xyz".to_string();
        println!("val = {:#?}", val);
    }

    for (key, val) in &map1 {
        println!("key: {} val: {}", key, val);
    }
}

pub fn btreemap_test() {
    println!("{}", "-----------BTreeMap_test--------------------");
    let mut map1 = BTreeMap::new();
    map1.insert(1, "val1");

    println!("map1: {:#?}", map1);
    assert_eq!(map1.get(&1), Some(&"val1"));
    assert_eq!(map1.get(&2), None);

    assert_eq!(map1.contains_key(&1), true);
    assert_eq!(map1.contains_key(&2), false);

    if let Some(x) = map1.get_mut(&1) {
        *x = "valx";
    }
    assert_eq!(map1.get(&1), Some(&"valx"));
    map1.remove(&1);
    assert_eq!(map1.get(&1), None);

    map1.insert(3, "val3");
    map1.insert(1, "val1");
    map1.insert(5, "val5");
    for (key, val) in map1.iter() {
        println!("key: {:#?}, val: {:#?}", key, val);
    }
    for (key, val) in map1.iter_mut() {
        *val = "xxx";
        println!("key: {:#?}, val: {:#?}", key, val);
    }
}

pub fn hashset_test() {
    println!("{}", "-----------HashSet_test--------------------");
    let mut set1 = HashSet::new();
    set1.insert(1);
    set1.insert(2);
    set1.insert(3);
    println!("set1: {:#?}", set1);

    assert_eq!(set1.contains(&1), true);
    assert_eq!(set1.len(), 3);
    assert_eq!(set1.is_empty(), false);

    for val in set1.iter() {
        println!("val: {:#?}", val);
    }

    for val in set1.drain() {
        println!("val: {:#?}", val);
    }
    assert_eq!(set1.is_empty(), true);
}

pub fn btreeset_test() {
    println!("{}", "-----------BTreeSet_test--------------------");
    let mut set1 = BTreeSet::new();
    set1.insert(1);
    set1.insert(2);
    set1.insert(3);
    println!("set1: {:#?}", set1);

    assert_eq!(set1.contains(&1), true);
    assert_eq!(set1.len(), 3);
    assert_eq!(set1.is_empty(), false);

    for val in set1.iter() {
        println!("val: {:#?}", val);
    }

    let mut set2 = BTreeSet::new();
    set2.insert(3);
    set2.insert(4);
    set2.insert(5);

    println!("diff: {:#?}", set1.difference(&set2));
    println!("symmetric_diff: {:#?}", set1.symmetric_difference(&set2));
    println!("intersection: {:#?}", set1.intersection(&set2));
    println!("union: {:#?}", set1.union(&set2));
    println!("is_disjoint: {:#?}", set1.is_disjoint(&set2));

    let mut set3 = BTreeSet::new();
    set3.insert(1);

    let mut set4 = BTreeSet::new();
    set4.insert(1);
    set4.insert(2);
    set4.insert(3);
    set4.insert(4);

    assert_eq!(set1.is_subset(&set4), true);
    assert_eq!(set1.is_superset(&set3), true);


}
