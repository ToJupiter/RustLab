use rust_lab::struct_enum::{define_enum, define_struct, module_demo, perm_struct};

#[test]
fn oxy_distance_is_pythagoras() {
    let p = define_struct::Oxy::x2_init(3.0, 4.0); // (6, 8)
    assert!((p.o_distance() - 10.0).abs() < 1e-9);
}

#[test]
fn oxy_is_further_and_setters() {
    let mut near = define_struct::Oxy::x2_init(1.0, 1.0);
    let far = define_struct::Oxy::x2_init(10.0, 10.0);
    assert!(far.is_further(&near));
    assert!(!near.is_further(&far));
    near.set_x(100.0);
    near.set_y(100.0);
    assert!(near.is_further(&far));
}

#[test]
fn struct_helpers_do_not_panic() {
    // User is private so we only assert these run without panic.
    let _ = define_struct::create_a_struct();
    let _ = define_struct::short_hand_init(
        String::from("a@x.com"),
        String::from("user"),
    );
    define_struct::print_oxy(&define_struct::Oxy::x2_init(1.0, 2.0));
    define_struct::oxy_ownership();
    define_struct::associated_and_method();
}

#[test]
fn enum_option_helpers_do_not_panic() {
    define_enum::create_ip();
    define_enum::option_type();
    define_enum::us_coins();
    define_enum::print_optional(Some(String::from("hi")));
    define_enum::print_optional(None);
    define_enum::print_optional_owned(Some(String::from("owned")));
    define_enum::print_optional_owned(None);
    define_enum::print_optional_borrowed(&Some(String::from("borrowed")));
    define_enum::print_optional_borrowed(&None);
    define_enum::option_string_print();
    define_enum::optional_if_let();
}

#[test]
fn perm_and_module_demos_do_not_panic() {
    perm_struct::borrow_and_set();
    perm_struct::ownership_manipulation();
    module_demo::eat_at_restaurant();
    module_demo::eat_breakfast();
}
