use learn::modules::collections;
use learn::modules::fs;
use learn::modules::path;
use learn::modules::time;
use learn::modules::result;

fn main() {
    collections::vec_test();
    collections::vecdeque_test();
    collections::linkedlist_test();
    collections::hashmap_test();
    collections::btreemap_test();
    collections::hashset_test();
    collections::btreeset_test();

    fs::dir_test();
    fs::file_test();
    
    path::path_test();

    time::time_test();
    
    result::result_test();
}
