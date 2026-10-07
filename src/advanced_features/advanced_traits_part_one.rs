use std::ops::Add;
/*
This chapter covers two related ideas:

1. Associated types — a placeholder type inside a trait that the implementor picks.
2. Default generic parameters — a generic parameter on a trait that has a fallback type, so users don't always have to specify it.

Both solve the same kind of problem: how do you write a trait that's flexible, without forcing users to type a lot?
 */
trait Container {
    type Item;
    fn first(&self) -> Option<&Self::Item>;
}

struct Numbers(Vec<i32>);
struct Words(Vec<String>);

impl Container for Numbers {
    type Item = i32;
    fn first(&self) -> Option<&i32> {
        return self.0.first();
    }
}

impl Container for Words {
    type Item = String;
    fn first(&self) -> Option<&String> {
        return self.0.first();
    }
}

pub fn associated_types_operator_overloading() {
    let word_container = Words(vec![String::from("Hello"), String::from("Rust")]);
    let number_container = Numbers(vec![10,20,30]);
    println!("The first of out words is: {:?}", word_container.first());
    println!("The first of out numbers is: {:?}", number_container.first());   

    // Operator overloading
    add_two_different_units();
}

/*
(+) is tied to the Add trait. Its real definition is:
 
``` rust
 trait Add<Rhs = Self> {
     type Output;
     fn add(self, rhs: Rhs) -> Self::Output;
 }
 ```
 Two things to notice:
 1, type Output; — an associated type for the result.
 2, Rhs = Self — a default generic parameter. Rhs is "right-hand side," and it defaults to Self.
 */
#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {x: i32, y: i32}

impl Add for Point {
    type Output = Point;
    fn add(self, other:Point) -> Point {
        Point {x: self.x + other.x, y: self.y + other.y}
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Milimeters(u32);

#[derive(Debug, Copy, Clone, PartialEq)]
struct Meters(u32);

impl Add<Meters> for Milimeters {
    type Output = Milimeters;
    fn add(self, other: Meters) -> Milimeters {
        Milimeters(self.0 + other.0 * 1000)
    }
}

fn add_two_different_units() {
    let p1 = Point {x: 102, y: 103};
    let p2 = Point {x: 105, y: 106};
    println!("Add 2 points p1 and p2, we got: {:?}", p1 + p2);

    let my_height: Milimeters = Milimeters(1800);
    let ultra_human_height = my_height + Meters(1);
    println!("Ultra human height is: {:?}", ultra_human_height);
}
