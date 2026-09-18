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
}