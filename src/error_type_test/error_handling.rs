use std::{fs::{File, OpenOptions, remove_file}, io::{ErrorKind, Write}};

use rand::Error;

/*
 * By default, these panics will print a failure message, unwind, clean up the stack, and quit. Via an environment variable, you can also have Rust display the call stack when a panic occurs to make it easier to track down the source of the panic.
 */
pub fn panicking() {
    // panic!("End right here, clear all of things");

    let v = vec![1,2,3,4];
    let invalid_lets_panic = v[100];
}

// enum Result<T, E> {
//     Ok(T), -> T is dtype when ok
//     Err(E), -> E is the error 
// }
pub fn file_open() {
    let greeting_file_result = File::open("src/error_type_test/test_file/hello.txt");
    // This does not exist, so we have to use OpenOptions to set the permissions if it is created
    let failed_file_result = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("src/error_type_test/test_file/hello_world.txt");

    // This will panic!
    // let failed_file_handler = match failed_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Cannot open the file: {error:?}")
    // };
    // 
    let mut failed_file_handler = match failed_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("src/error_type_test/test_file/hello_world.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Cannot create file with error: {e:?}")
            },
            _ => panic!("Problem opening file with error: {error:?}")
        },
    };
    let write_to_hello_world = failed_file_handler.write_all(String::from("hello world from rust lang").as_bytes());
    println!("Complete or not? {:?}", write_to_hello_world);

    // Will fail cause immutable
    // let greeting_write = greeting_txt.write_all(String::from("hello world from rust lang").as_bytes());
    
}

pub fn file_lifecycle() {
    let mut hello_rust = File::open("src/error_type_test/test_file/hello_rust.txt");
    let mut hello_rust_handler = match hello_rust {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("src/error_type_test/test_file/hello_rust.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Cannot create file with error: {e:?}")
            },
            _ => panic!("Problem opening file with error: {error:?}")
        },
    };
    let mut hello_rust_write = &hello_rust_handler.write("Hello from Rust programming language".as_bytes());
    println!("The result for writing to hello rust is {:?}", hello_rust_write);

    let hello_rust_remove = match remove_file("src/error_type_test/test_file/hello_rust.txt") {
        Ok(okay) => String::from("Removal complete"),
        Err(error) => String::from("Error removing the file")
    };
    println!("The removal cleanup process is: {}", hello_rust_remove);
}

/*
 * Code => value that do not make sense => return an error. This is for retry, handling, ... and the part in the sense of userspace error plane.
 * Insecure, harmful => panic! => for developer to fix
 * Expected failure => Result<T, E>
 */
