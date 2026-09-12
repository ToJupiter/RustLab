use std::fmt::{Debug, Display};

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
    println!(
        "The new way to find the largest value returns {:?}",
        new_largest
    );
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
pub fn largest<T: PartialOrd>(list: &[T]) -> &T {
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
    y: T,
}

pub fn sample_point() {
    let integer_impl = Point { x: 5, y: 10 };
    let float_impl = Point { x: 1.0, y: 4.0 };

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p1 = SomeCoordinates { x: 10, y: 20.0 };
    let p2 = SomeCoordinates {
        x: "Hello",
        y: 10_000,
    };

    let a_sample_cat = Cat {
        cat_type: String::from("USA Cat"),
        color: String::from("orange"),
        meow: String::from("Meow"),
    };

    let a_sample_dog = Dog {
        color: String::from("black"),
        dog_type: String::from("idk"),
        weight: 10.5,
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

impl<X1, Y1> SomeCoordinates<X1, Y1> {
    fn mixup<X2, Y2>(self, other: SomeCoordinates<X2, Y2>) -> SomeCoordinates<X1, Y2> {
        SomeCoordinates {
            x: self.x,
            y: other.y,
        }
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
    pub content: String,
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
    pub time_and_date: String,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        return format!(
            "Posted by {} in {}, location_id {}",
            self.user, self.time_and_date, self.location
        );
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
    pub weight: f64,
}

#[derive(Debug)]
pub struct Cat {
    pub cat_type: String,
    pub color: String,
    pub meow: String,
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
        return f
            .debug_struct("Dog")
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

pub fn i32_function<T, U>(t: &T, u: &U) -> i32
where
    T: Summary + Clone,
    U: Bark + Debug,
{
    return 0;
}

/* If return impl Trait, then only return one type of struct, enum */
pub fn dog_cat_barking<T>(home_animal: &T) -> impl Bark {
    return Dog {
        dog_type: String::from("golden retriever"),
        color: String::from("golden"),
        weight: 25.5,
    };
}

/* One struct/ enum can have multiple impl for each case */
struct Minh<T> {
    school: T,
    height: u64,
    weight: u64,
    hometown: T,
}

impl<T> Minh<T> {
    pub fn new_minh(school: T, height: u64, weight: u64, hometown: T) -> Self {
        return Self {
            school,
            height,
            weight,
            hometown,
        };
    }
}

impl<T: Display + PartialOrd> Minh<T> {
    pub fn display_minh(&self) {
        if self.school >= self.hometown {
            println!("Truong lon hon que");
        } else {
            println!("Que lon hon truong");
        }
    }
}

impl<T: Display> Bark for T {
    fn make_sound(&self) -> String {
        return String::from("T woof woof woof");
    }
}

pub fn testing_crazy_impl() {
    let home_animal = 10;
    dog_cat_barking(&home_animal);

    let new_dog = Dog {
        dog_type: String::from("golden retriever"),
        color: String::from("golden"),
        weight: 30.0,
    };
    new_dog.make_sound();
}

fn displayable<T: Display>(t: T) -> impl Display {
    t
}

pub fn string_display() {
    let s = String::from("hello");
    let mut s2 = displayable(s);
    // If the type impl the Display trait, we can use to_string() method to get it back to String. &Sized trait also.
    let mut s2 = s2.to_string();
    s2.push_str(" world");
    println!("{s2}");
}

/*
 * Lifetime of variables and scopes in Rust
 */
fn r_outlives_x() {
    let r = 10; // ---------+-- 'a
    //          |
    // {                     //          |
    //     let x = 5;        // -+-- 'b  |
    //     r = &x;           //  |       |
    // }                     // -+       |
    //          |
    println!("r: {r}"); //          |
} // ---------+

/* Since x outlives r, it is valid for r to borrow x. */
fn x_outlives_r() {
    let x = 5; // ----------+-- 'b
    //           |
    let r = &x; // --+-- 'a  |
    //   |       |
    println!("r: {r}"); //   |       |
    // --+       |
} // ----------+

pub fn longest_string_slice<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    } else {
        return y;
    }
}

/* Now with 2 lifetimes specified, we could manipulate this. We ensure that the lifetime of the scope and x matches. */
pub fn lifetime_b_string_slice<'b, 'a>(x: &'b str, y: &'a str) -> &'b str {
    return x;
}

pub fn valid_vs_invalid_lifetime() {
    let first_string = String::from("Hello Rust programming language");

    {
        let second_string = String::from("hello");
        let longer_str = longest_string_slice(first_string.as_str(), second_string.as_str());
        println!("The longer string out of this 2 is: {:?}", longer_str);
    }

    let third_string = String::from("if this lives longer");
    let second_longer;

    {
        let fourth_string = String::from(" then it will be invalid");
        second_longer = longest_string_slice(third_string.as_str(), fourth_string.as_str());
    }

    // fourth_string does not live long enough, second_longer will be destroyed in the above scope. Even if the third_string may be longer, the compiler could not risk it for unsafe code.
    // println!("The longer string is: {}", second_longer);
    //

    let a_paragraph =
        String::from("This is a sample paragraph: \n In New York there is Wall Street");
    let a_paragraph_title = String::from("Wall St. in New York");
    let a_paragraph_struct = AParagraph {
        paragraph: a_paragraph.as_str(),
        title: a_paragraph_title.as_str(),
    };

    // Longest with an announcement
    let para_vs_title = longest_with_an_announcement(
        a_paragraph.as_str(),
        a_paragraph_title.as_str(),
        String::from("Announcement: Paragraph Length vs Title Length"),
    );
}

// Lifetime annotation also impacts struct definition
struct AParagraph<'a> {
    paragraph: &'a str,
    title: &'a str,
}

trait PrintParagraph {
    fn print_title<'a>(&'a self) -> &'a str {
        return "Sample title";
    }

    fn print_paragraph<'b>(&'b self) -> &'b str {
        return "Sample paragraph";
    }
}

// So we can implement it like this. Please mote that everything should have lifetime specifier or none should have since compiler auto-infer.
impl<'s> PrintParagraph for AParagraph<'s> {
    fn print_paragraph<'b>(&'b self) -> &'b str {
        return self.paragraph;
    }

    fn print_title<'a>(&'a self) -> &'a str {
        return self.title;
    }
}

// This can run without lifetime variables because compiler is smart enough
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

/* The 3 rules on lifetime:
* The first rule is that the compiler assigns a different lifetime parameter to each lifetime in each input type. References like &'_ i32 need a lifetime parameter, and structures like ImportantExcerpt<'_> need a lifetime parameter. For example:

    The function fn foo(x: &i32) would get one lifetime parameter and become fn foo<'a>(x: &'a i32).
    The function fn foo(x: &i32, y: &i32) would get two lifetime parameters and become fn foo<'a, 'b>(x: &'a i32, y: &'b i32).
    The function fn foo(x: &ImportantExcerpt) would get two lifetime parameters and become fn foo<'a, 'b>(x: &'a ImportantExcerpt<'b>).

The second rule is that, if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.

The third rule is that, if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters. This third rule makes methods much nicer to read and write because fewer symbols are necessary.
*/

/*
 * Most of the time, an error message suggesting the 'static lifetime results from attempting to create a dangling reference or a mismatch of the available lifetimes. In such cases, the solution is to fix those problems, not to specify the 'static lifetime.
 */
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display + Debug,
{
    println!("The announcement is: {:?}", ann);
    if x.len() > y.len() { x } else { y }
}

/* Some important Rust quizzes thing to think about:
 * &[T].sort() does not free the elements inside the slixw
 * This below is the correct way of manipulating a slice of &[T].
 */
fn find_nth<T: Ord + Clone>(elems: &[T], n: usize) -> T {
    let mut elem_refs: Vec<&T> = elems.iter().collect();
    elem_refs.sort();
    let t = elem_refs[n];
    return t.clone();
}

/*
 * Context: By inlining the definition of get_curve into apply_curve, the borrow checker understands that self.curve is not self.scores, so it allows the function to compile. This is a common workaround for this type of borrow checker limitation.
 
 Another option is to leverage the fact that self.curve is cheap to copy and use Option::copied, which would release the borrow on self as soon as .copied() is called.
 */
// pub fn apply_curve(&mut self) {
//     if let Some(curve) = self.curve {
//         for score in self.scores.iter_mut() {
//             *score += curve;
//         }
//     }
// }