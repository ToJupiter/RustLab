use rust_lab::error_type_test::{error_handling, generic_type, test_write};

#[test]
fn find_largest_picks_max() {
    let list = vec![100, 38, 20, 99];
    assert_eq!(*generic_type::find_largest(&list), 100);
    let list2 = vec![3, 1, 9, 2];
    assert_eq!(*generic_type::find_largest(&list2), 9);
}

#[test]
#[should_panic]
fn find_largest_panics_on_empty() {
    let empty: Vec<i32> = vec![];
    generic_type::find_largest(&empty);
}

#[test]
fn longest_slice_returns_longer() {
    assert_eq!(generic_type::longest_string_slice("abc", "ab"), "abc");
    assert_eq!(generic_type::longest_string_slice("ab", "abc"), "abc");
}

#[test]
fn bark_sounds_for_cat_and_dog() {
    let cat = generic_type::Cat {
        cat_type: String::from("tabby"),
        color: String::from("gray"),
        meow: String::from("Meow"),
    };
    let dog = generic_type::Dog {
        dog_type: String::from("lab"),
        color: String::from("black"),
        weight: 20.0,
    };
    assert_eq!(generic_type::only_cat_and_dogs(&cat), "meow meow");
    assert_eq!(generic_type::only_cat_and_dogs(&dog), "grr gau gau");
}

#[test]
fn summary_and_bark_helpers() {
    #[derive(Clone)]
    struct Dummy;
    impl generic_type::Summary for Dummy {
        fn summarize(&self) -> String {
            String::from("dummy")
        }
    }
    let dummy = Dummy;
    let dog = generic_type::Dog {
        dog_type: String::from("lab"),
        color: String::from("black"),
        weight: 20.0,
    };
    assert!(generic_type::i32_function(&dummy, &dog) == 0);
    // dog_cat_barking returns impl Bark; verify default sound path via make_sound
    use rust_lab::error_type_test::generic_type::Bark;
    let animal = generic_type::dog_cat_barking(&10);
    assert_eq!(animal.make_sound(), "grr gau gau");
}

#[test]
fn generic_demos_do_not_panic() {
    generic_type::vector_largest();
    generic_type::sample_point();
    generic_type::testing_crazy_impl();
    generic_type::string_display();
    generic_type::valid_vs_invalid_lifetime();
}

#[test]
fn greeting_formats_name() {
    assert_eq!(test_write::greeting("icky"), "Say hello to icky");
}

#[test]
fn file_lifecycle_creates_and_removes_tmp_file() {
    // file_lifecycle() uses repo-relative paths
    // (src/error_type_test/test_file/hello_rust.txt) and always removes
    // the file, so run it from the crate root and assert cleanup.
    std::fs::create_dir_all("src/error_type_test/test_file").unwrap();
    error_handling::file_lifecycle();
    assert!(!std::path::Path::new("src/error_type_test/test_file/hello_rust.txt").exists());
}
