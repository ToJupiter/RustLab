use rust_lab::borrow_checker_simple::{borrow, easy, fix_unsafe};
use std::rc::Rc;

#[test]
fn easy_demos_do_not_panic() {
    // borrow_checking() builds a 1M-element array on the stack before
    // boxing it, overflowing the default 2 MiB test-thread stack, so run
    // it on a thread with a bigger stack.
    std::thread::Builder::new()
        .stack_size(8 * 1024 * 1024)
        .spawn(easy::borrow_checking)
        .unwrap()
        .join()
        .unwrap();
    easy::ownership_between();
    easy::borrow_not_owned();
    easy::deref_data();
    easy::deref_exp();
}

#[test]
fn borrow_demos_do_not_panic() {
    borrow::simple_ref_arr();
    borrow::compare_perms();
    borrow::vector_borrow();
    borrow::mutable_ref();
    borrow::mutable_borrowing();
}

#[test]
fn destroyed_string_moves_ownership() {
    assert_eq!(
        borrow::destroyed_or_own_string(),
        "Hello the Rust language"
    );
}

#[test]
fn static_str_literal() {
    assert_eq!(borrow::string_literal_return(), "Hello Rust language");
}

#[test]
fn rc_string_clone_points_to_same_value() {
    let rc: Rc<String> = borrow::rc_string_return();
    assert_eq!(&*rc, "Hello from Rust lang");
}

#[test]
fn caller_provided_string_is_filled_in_place() {
    let mut out = String::new();
    borrow::caller_create_string_return(&mut out);
    assert_eq!(out, "Hello from Rust lang");
}

#[test]
fn first_word_finds_space_or_len() {
    assert_eq!(fix_unsafe::first_word(&String::from("Hello world")), 5);
    assert_eq!(fix_unsafe::first_word(&String::from("Hello")), 5);
    assert_eq!(fix_unsafe::first_word(&String::from("")), 0);
}

#[test]
fn add_big_strings_pushes_only_longer_ones() {
    let mut dst = vec![String::from("hi")]; // longest len = 2
    let list = vec![String::from("a"), String::from("hello world")];
    fix_unsafe::add_big_strings_together(&mut dst, &list);
    assert_eq!(dst, vec![String::from("hi"), String::from("hello world")]);
}

#[test]
fn array_element_and_slices_do_not_panic() {
    fix_unsafe::array_element();
    fix_unsafe::string_slice();
    fix_unsafe::string_return();
    fix_unsafe::first_word_as_bytes(&String::from("Hello world"));
    fix_unsafe::type_conversion();
}
