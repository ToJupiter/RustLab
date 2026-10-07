use crate::struct_enum::define_enum::print_optional_owned;

/*
match VALUE {
     PATTERN => EXPRESSION,
     PATTERN => EXPRESSION,
     PATTERN => EXPRESSION,
 }

 let PATTERN = EXPRESSION;

 In the expression if let Some(x) = a_value, then Some(x) is refutable. If the value in the a_value variable is None rather than Some, the Some(x) pattern will not match.
 */
pub fn if_let() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }


    let x = Some(50);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Got {}", y),
        _ => println!("Default case, x={:?}", x)
    }
    println!("at the end: x = {x:?}, y = {y}");

    match x {
        Some(1|2) => println!("One or two"),
        Some(3) => println!("Three"),
        _ => println!("Anything else")
    }

    match x {
        Some(1..=5) => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'd';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // Break apart values inside the statement
    let p = Point {x: 10, y: 20};
    let Point {x: a, y: b} = p;
    assert_eq!(10, a);
    assert_eq!(20, b);

    // We can use this to check for in the 2D space
    let p2 = Point { x: 0, y: 7 };
    match p2 {
        Point {x, y: 0} => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    matching_color_message();
    the_at_operator();
}

struct Point {
    x: i32, y: i32
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn matching_color_message() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}");
        }
        _ => (),
    }
}

fn the_at_operator() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id @ 3..=7 } => {
            println!("Found an id in range: {id}")
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {id}"),
    }

}