use std::{io::ErrorKind::InvalidInput, ops::Deref};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(t: T) -> MyBox<T> {
        return MyBox(t);
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

fn say_hello(input: &str) {
    println!("Hello, {}", input);
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

pub fn cons_list() {
    let boxed_i32 = Box::new(10);
    let gangs = 100;

    assert_eq!(*boxed_i32, 10);

    let my_box_i32 = MyBox::new(10);
    // assert_eq!(*my_box_i32, 10); will fail because not impl Deref

    let input = String::from("from Rust lang");
    let string_test_impl = say_hello(input.as_str());

    let second_input = MyBox::new(String::from("from planet Earth"));
    let string_test_impl = say_hello(&(*second_input)[..]);
    println!("The second input is: {:?}", string_test_impl);

    // Customize Pie
    let example_pie = Pie {
        shape: String::from("rounded"),
        height: 180.5
    };
    println!("We will intefere with the drop here and drop it before scope ends: {:?}", drop(example_pie));
}

struct Pie {
    shape: String,
    height: f64
}

impl Drop for Pie {
    fn drop(&mut self) {
        println!("Dropping Pie with data: {} and {}!", self.shape, self.height);
    }
}



