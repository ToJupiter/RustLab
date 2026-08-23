use std::ptr::read;

// If you use the keyword let, you can simply do anything
pub fn shadowing () {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x inside this is: {}", x);
    }

    println!("The real value of x in the shadowing scope is: {}", x);
}

// You cannot cast a variable directly to other types. That violates.
pub fn char_in_rust() {
    let char_z : char = 'z';
    println!("This is the z char {char_z}");
}

// Tuple in Rust is quite easy to work with
pub fn this_is_tuple(a: u64, b: i64, c: u64) -> (u64, i64, u64) {
    let tuple_abc : (u64, i64, u64) = (a,b,c);
    let a_access = tuple_abc.0;
    let b_access = tuple_abc.1;
    let c_acesss = tuple_abc.2;
    return (a_access, b_access, c_acesss);
}

// The array type; fixed array or let arr: [i32; num_elements] = []
// let arr = [num; replication_factor]
pub fn sample_arr() {
    let arr = [1,2,3,4,5];
    let second_arr: [i32; 3] = [1,12,3];
    let third_arr = [3;5];
}

// Test plus_one
fn plus_one(x: i32) -> i32 {
    x + 1
}

pub fn test_x_case(){
    let x = plus_one(5);
    println!("{x}");

    let y = 5;
    let z = plus_one(y);
    println!("{y}");
}

pub fn im_stupid_if_else () {
    let number = 3;

    if (number < 5) {
        println!("Less than 5");
    } else {
        println!("Bigger than 5");
    }

    // "If" is a type of expr
    let condition = true;
    let number_two: i32 = if condition {5} else {7};
}

pub fn stupid_loop () {
    // Why do we have break counter * 2
    let mut counter = 0;
    
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result after the infinite loop is {result}");

    // Loops can have labels,but I do not quite understand this sh
    let mut count = 0;
    'counting_up : loop {
        println!("Count = {count}");
        let mut remaining = 10;

        loop {
            println!("Remaining {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count: {count}");
}

pub fn for_types() {
    let arr = [10, 20, 30, 40, 50];

    for element in arr {
        println!("The value is: {element}");
    }

    for number in (1..4).rev() {
        println!("Revision number prints {number}");
    }
}