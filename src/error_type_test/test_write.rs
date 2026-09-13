pub fn greeting(name: &str) -> String {
    return format!("Say hello to {}", name);
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn greeting_contain_name() {
        let name_to_greet = String::from("Micky");
        assert!(greeting(name_to_greet.as_str()).contains("icky"), "Greeting did not contain icky, the string is: {}", name_to_greet);
    }

    #[test]
    #[should_panic(expected="I just panicked!")]
    fn should_panic_btw() {
        panic!("I just panicked!");
    }

    #[test]
    #[ignore = "Not useful"]
    // cargo test -- --ignored
    pub fn ignored_test() {
        assert_eq!(2+2, 4);
    }
}

/* Finding all tests containing the 'add' sub-string:
 * $ cargo test add
    Compiling adder v0.1.0 (file:///projects/adder)
     Finished `test` profile [unoptimized + debuginfo] target(s) in 0.61s
      Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)
 
 running 2 tests
 test tests::add_three_and_two ... ok
 test tests::add_two_and_two ... ok
 
 test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s
 */

