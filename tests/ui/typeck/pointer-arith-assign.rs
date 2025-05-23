//@ run-rustfix
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

fn test_add_assign_raw_pointer() {
    let mut arr = [0u8; 10];
    let mut _ptr = arr.as_mut_ptr();

    _ptr += 2; //~ ERROR binary assignment operation `+=` cannot be applied to type `*mut u8` [E0368]
}

fn test_sub_assign_raw_pointer() {
    let mut arr = [0u8; 10];
    let mut _ptr = arr.as_mut_ptr();

    _ptr -= 2; //~ ERROR binary assignment operation `-=` cannot be applied to type `*mut u8` [E0368]
}

fn main() {}
