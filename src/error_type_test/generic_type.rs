use std::fmt::Debug;

pub fn vector_largest() {
    let number_list = vec![100, 38, 20, 99];

    let mut largest_num = &number_list[0];
    for number in &number_list {
        if number > largest_num {
            largest_num = number;
        }
    }
    println!("The largest number is: {:?}", largest_num);

    // The correct way to do an abstraction
    let new_number_list = vec![1000, 29, 38, 20];
    let new_largest = find_largest(&new_number_list);
    println!("The new way to find the largest value returns {:?}", new_largest);
}

/* This is an abstraction of the operation to find the largest element in a vector */
pub fn find_largest(list: &[i32]) -> &i32 {
    let mut largest_num = &list[0];

    for item in list {
        if item > largest_num {
            largest_num = item;
        }
    }
    return largest_num;
}

/*
 * PartialOrd limits the trait of the generic type T
 */
pub fn largest<T: PartialOrd> (list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item == largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T
}

pub fn sample_point() {
    let integer_impl = Point {x: 5, y: 10};
    let float_impl = Point {x: 1.0, y: 4.0};

    let p = Point {x: 5, y: 10};
    println!("p.x = {}", p.x());

    let p1 = SomeCoordinates {x: 10, y: 20.0};
    let p2 = SomeCoordinates {x: "Hello", y: 10_000};

    let a_sample_cat = Cat {
        cat_type: String::from("USA Cat"),
        color: String::from("orange"),
        meow: String::from("Meow")
    };

    let a_sample_dog = Dog {
        color: String::from("black"),
        dog_type: String::from("idk"),
        weight: 10.5
    };


}

impl<T> Point<T> {
    fn x(&self) -> &T {
        return &self.x;
    }
}

impl Point<f32> {
    fn distance_to_core(&self) -> f32 {
        return (self.x.powi(2) + self.x.powi(2)).sqrt();
    }
}

struct SomeCoordinates<X1, Y1> {
    x: X1,
    y: Y1,
}

impl <X1, Y1> SomeCoordinates<X1, Y1> {
    fn mixup<X2, Y2>(self, other: SomeCoordinates<X2, Y2>) -> SomeCoordinates<X1, Y2>{
        SomeCoordinates { x: self.x, y: other.y }
    }
}

/*
 * Trait works exactly like how interfaces do in other languages
 */
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        return format!("{}, by {} in {}", self.headline, self.author, self.location);
    }
}

pub struct SocialPost {
    pub user: String,
    pub content: String,
    pub location: String,
    pub time_and_date: String
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        return format!("Posted by {} in {}, location_id {}", self.user, self.time_and_date, self.location);
    }
}

/*
 * If this default behavior is overridden, we cannot call it later.
 * Animals start here.
 */
pub trait Bark {
    fn make_sound(&self) -> String {
        return String::from("wolf, wolf, wolf");
    }
}

pub struct Dog {
    pub dog_type: String,
    pub color: String,
    pub weight: f64
}

#[derive(Debug)]
pub struct Cat {
    pub cat_type: String,
    pub color: String,
    pub meow: String
}

impl Bark for Dog {
    fn make_sound(&self) -> String {
        return String::from("grr gau gau");
    }
}

impl Bark for Cat {
    fn make_sound(&self) -> String {
        return String::from("meow meow");
    }
}

impl Debug for Dog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return f.debug_struct("Dog")
            .field("cat_type", &self.dog_type)
            .field("color", &self.color)
            .field("weight", &self.weight)
            .finish();
    }
}


pub fn only_cat_and_dogs<T: Bark + Debug>(input_animal: &T) -> String {
    println!("Debug print: {:?}", input_animal);
    return input_animal.make_sound();
}
