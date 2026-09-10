use std::cmp::Ordering;
use std::{error, io};
mod first_chap;
mod var_and_mut;
mod borrow_checker_simple;
mod struct_enum;
mod collections_dtype;
mod error_type_test;

use rand::Rng;

fn main() {
    // first_chap::matching_guess();
    // var_and_mut::varMut::shadowing();
    // var_and_mut::varMut::test_x_case();
    // var_and_mut::varMut::stupid_loop();
    // 
    // borrow_checker_simple::easy::deref_exp();
    // borrow_checker_simple::borrow::mutable_ref();
    // borrow_checker_simple::fix_unsafe::first_word_as_bytes(&String::from("Hello world"));
    // struct_enum::define_struct::oxy_ownership();
    // 
    // struct_enum::define_enum::us_coins();
    // collections_dtype::vec_str::vector_iter();
    collections_dtype::vec_str::string_manipulation();
    error_type_test::error_handling::file_lifecycle();
    error_type_test::generic_type::vector_largest();
}   

