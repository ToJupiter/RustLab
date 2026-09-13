use rust_lab::first_chap;

#[test]
fn a_plus_b_basic() {
    assert_eq!(first_chap::a_plus_b(2, 3), 5);
    assert_eq!(first_chap::a_plus_b(-1, 1), 0);
    assert_eq!(first_chap::a_plus_b(0, 0), 0);
}

#[test]
fn string_conversion_trims_and_parses() {
    assert_eq!(first_chap::string_conversion_to_types("42".to_string()), 42);
    assert_eq!(
        first_chap::string_conversion_to_types("  -7\n".to_string()),
        -7
    );
}

#[test]
#[should_panic]
fn string_conversion_panics_on_non_numeric() {
    first_chap::string_conversion_to_types("hello".to_string());
}
