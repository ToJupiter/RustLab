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
    let add_one_v3 = |m| {m as f32 + 1.0};
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

    let a_fun_arr_move_completely = thread::spawn(move || println!("From thread: {:?}", a_fun_arr)).join().unwrap();
}

/*
 * FnOnce -> fn call(self) -- (consume — may move out, can only be done once)
 * FnMut -> fn call(&mut self) -- (borrow mutably - may mut, does not move out)
 * Fn -> fn call(&self) -- (borrow shared, may only read, mutate, not move)

This example will demonstrate how the callers work.
 */

fn caller_shared_borrow<F: Fn()>(f: &F) { f(); f(); }
fn caller_shared_borrow_mut<F: FnMut()>(f: &mut F) { f(); f(); }
fn caller_once_take_ownership<F: FnOnce()>(f: F) { f(); }

pub fn closure_levels() {

    // Greetings!
    let name = String::from("Alice");
    let mut greet_closure = || println!("Hello, {}", name);
    // greet calls: No moving, no mutating value
    caller_shared_borrow(&greet_closure);
    caller_shared_borrow_mut(&mut greet_closure);
    caller_once_take_ownership(greet_closure);

    
    let mut counter = 0;
    let mut bump_value = || {counter += 1; println!("Current count: {}", counter)};
    // counter calls: No moving, YES mutating value. bump_value only implements FnMut, no Fn implementation.
    // caller_shared_borrow(&bump_value); -- failed because no Fn implementation.
    caller_shared_borrow_mut(&mut bump_value);
    caller_once_take_ownership(bump_value);

    let mut rust_string = String::from("Hello Rust lang");
    let eat_rust_string = move || drop(rust_string);

    // caller_shared_borrow(&eat_rust_string); -- failed because no Fn implementation.
    // caller_shared_borrow_mut(&mut eat_rust_string);
    caller_once_take_ownership(eat_rust_string);
}

pub fn popular_example() {

    // -- This accepts FnOnce - meaning you are free to move, mut or do anything. This is because unwrap_or_else here only handles Some or None. If Some() -> call 0 time, elif None -> call once only. It grants maximum flexibility to unwrap_or_else().
    let fallback_noti = String::from("Fallback to this String");
    let x: Option<String> = None;
    let lets_fallback = x.unwrap_or_else(move || fallback_noti); 

    // -- The map function needs at least FnMut, does not accept FnOnce. 
    let ai_test_scores: Vec<f64> = vec![5.5, 7.5, 6.5, 5.5, 2.5];
    let (min_test_score, max_test_score) = (ai_test_scores.iter().copied().reduce(f64::min), ai_test_scores.iter().copied().reduce(f64::max));
    let (min_test_score_unwrap, max_test_score_unwrap) = (min_test_score.unwrap(), max_test_score.unwrap());
    let min_max_closure = |x| {x - min_test_score_unwrap / (max_test_score_unwrap - min_test_score_unwrap)};
    let min_max_normalization: Vec<f64> = ai_test_scores.iter().map(min_max_closure).collect();
    
    
}

