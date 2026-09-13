use rust_lab::collections_dtype::vec_str;
use rust_lab::var_and_mut::varMut;

#[test]
fn tuple_echoes_inputs() {
    assert_eq!(varMut::this_is_tuple(1, -2, 3), (1, -2, 3));
}

#[test]
fn var_mut_demos_do_not_panic() {
    varMut::shadowing();
    varMut::char_in_rust();
    varMut::sample_arr();
    varMut::test_x_case();
    varMut::im_stupid_if_else();
    varMut::stupid_loop();
    varMut::for_types();
}

#[test]
fn vec_and_string_demos_do_not_panic() {
    vec_str::creating_vector();
    vec_str::vector_iter();
    vec_str::enum_vector();
    vec_str::string_manipulation();
    vec_str::hashmap_manipulation();
}
