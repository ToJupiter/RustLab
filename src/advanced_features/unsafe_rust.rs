use std::slice;

/*
    Dereference a raw pointer.
    Call an unsafe function or method.
    Access or modify a mutable static variable.
    Implement an unsafe trait.
    Access fields of unions.
 */
pub fn raw_pointer() {
    let mut test_number = 1005;

    let r1 = &raw const test_number;
    let r2 = &raw mut test_number;

    unsafe {
        println!("Raw pointer r1 deref: {}", *r1);
        println!("Raw pointer r2 deref: {}", *r2);
        unsafe_operations();
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // Own block for safe wrapper testing
    let mut v = [1, 2, 3, 4, 5];
    let (left, right) = split_at_mut_i32(&mut v, 2);
    println!("left  = {:?}", left);
    println!("right = {:?}", right);
    left[0] = 10;
    right[2] = 50;
    println!("after mutation: {:?}", v);
}

unsafe fn unsafe_operations() {
    let mut a_sample_slice: &str = "Say hello to the Rust programming language";

    let raw_pointer_slice_1 = &raw const a_sample_slice;
    let raw_pointer_slice_2 = &raw mut a_sample_slice;

    unsafe {
        println!("Raw pointer slice 1 deref: {}", *raw_pointer_slice_1);
        println!("Raw pointer slice 2 deref: {}", *raw_pointer_slice_2);
    }
    
}

fn split_at_mut_i32(input_slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = input_slice.len();
    let input_slice_ptr = input_slice.as_mut_ptr();

    unsafe {
        return (
            slice::from_raw_parts_mut(input_slice_ptr, mid),
            slice::from_raw_parts_mut(input_slice_ptr.add(mid), len - mid)
        );
    }
}

unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

