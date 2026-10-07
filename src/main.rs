use std::cmp::Ordering;
use std::{error, io};
mod first_chap;
mod var_and_mut;
mod borrow_checker_simple;
mod struct_enum;
mod collections_dtype;
mod error_type_test;
mod functional_pointer;
mod concurrency_async;

use rand::Rng;
use rust_lab::{advanced_features, pattern_matching};



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
    // collections_dtype::vec_str::string_manipulation();
    // error_type_test::error_handling::file_lifecycle();
    // error_type_test::generic_type::vector_largest();
    // functional_pointer::smart_pointer::cons_list();
    // functional_pointer::rc_pointer::rc_pointer_test();
    // concurrency_async::concurrency::thread_spawn();
    // concurrency_async::concurrency::message_passing_channel();
    // trpl::block_on(async { 
    //     concurrency_async::hello_async::welcome_async::page_title("https://example.com").await;
    // });
    // concurrency_async::hello_async::welcome_async::start_without_async();
    // concurrency_async::hello_async::welcome_async::calling_async_exec();
    pattern_matching::pattern_matching_intro::if_let();
    advanced_features::unsafe_rust::raw_pointer();
    advanced_features::advanced_traits_part_one::associated_types_operator_overloading();
    advanced_features::advanced_traits_part_two::part_two();
}

