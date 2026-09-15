use std::{thread, time::Duration};

pub fn type_of_closure() {
    let expensive_closure = |num_u32 : u32 | {
        println!("Calcultating the values inside of the closure: {:?}", num_u32);
        thread::sleep(Duration::from_secs(2));
        return num_u32 + 1;
    };

    let add_one_v2 = |x: i32| -> i32 {
        return x + 1;
    };
    let add_one_v3 = |m| {m + 1};
    let add_one_v4 = |n| n + 1;

    let add_one_v3_applied = add_one_v3(10);
    let add_one_v4_applied = add_one_v4(20);

    // let add_one_v3_failed = add_one_v3(12.0); // will fail because of compiler-inffered dtype is i32

    /* Ownership when using closure */
    // Only borrowing
    let mut a_fun_arr: Vec<i32> = vec![10, 20, 30];

    println!("Before borrowing: {:?}", a_fun_arr);
    let a_fun_borrow = || println!("The fun array is: {:?}", a_fun_arr);

    println!("Before calling borrow-only: {:?}", a_fun_arr);
    a_fun_borrow();
    println!("After calling borrow-only: {:?}", a_fun_arr);

    // Mutable borrowing
    let mut a_fun_arr_borrow_mut = || println!("The 50 push: {:?}", a_fun_arr.push(50));

    
}