use std::f32::consts::{FRAC_PI_3, FRAC_PI_4};
use std::{slice::Iter};
use std::ops::Range;


pub fn creating_vector(){
    let v: Vec<i32> = Vec::new();
    let mut v = vec![1,2,3];

    v.push(10);
    v.push(20);

    let third_element = &v[2];
    println!("The third element of v is {}", third_element);

    // The get method returns Option<&T> instead of Option<T>. Be careful with the & notation here.
    let third_element_option: Option<&i32> = v.get(2);
    match third_element_option {
        Some(third) => println!("The third element of the vector is {third}"),
        None => println!("None!")
    }

    // let does_not_exist = &v[100]; panic! because does not exist

    // This returns None if there is nothing
    let does_not_exist = v.get(100);

    // Iterate through the vector
    for element in &v {
        println!("{} ", element);
    }

    for element in &mut v {
        *element += 50;
    }
}

pub fn vector_iter() {
    let mut v: Vec<i32>         = vec![10, 20];
    let mut iter: Iter<'_, i32> = v.iter();

    // Result: Iter([v[0], v[1]])
    println!("Let's debug the iter: {:?}", iter);
    // n1 points to first element : v[0]
    let n1: &i32                = iter.next().unwrap();
    // n2 points to second element: v[1]
    let n2: &i32                = iter.next().unwrap();
    // iter.next() points to obsolete, so if we do unwrap() here then panic!
    let end: Option<&i32>       = iter.next();


    // Second vector. Iter takes the perms from v
    let mut v2: Vec<i32> = vec![100,200,300,400];
    let mut iter: Range<usize> = 0..v2.len();
    let i1: usize = iter.next().unwrap();
    // i1 = 100, because Range<idx> {"start": 0, "end": v2.len()}
    let n1: &i32 = &v[i1];
}

/* Enum: allowance for different types in vector */
enum Coordinates {
    Oxy(f32, f32),
    Oxyz(f64, f64, f64),
    Circle(f32, f32),
    Spherical(f64, f32, f32)
}

pub fn enum_vector() {
    let random_vectors = vec![
        Coordinates::Oxy(1.0, 10.5),
        Coordinates::Oxyz(10.0, 20.0, 30.0),
        Coordinates::Spherical(10.0, FRAC_PI_3, FRAC_PI_4)
    ];
}
