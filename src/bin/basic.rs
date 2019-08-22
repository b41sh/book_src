use learn::basic::variables;
use learn::basic::data_types;
use learn::basic::ownership;
use learn::basic::structs;
use learn::basic::enums;

fn main() {
    variables::var_test();
    data_types::scalar_test();
    data_types::compound_test();

    ownership::scope_test();
    ownership::ownership_test();
    ownership::ownership_fn_test();
    ownership::references_test();
    ownership::slice_test();

    structs::struct_test();

    enums::enum_test();
}

