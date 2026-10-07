/* Disambiguating Between Identically Named Methods */
pub fn part_two() {
    identically_named_method();
    supertrait_demo();
}

/* Human */
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

/* Other types */
trait Container {
    fn describe() -> String;
}

struct DockerContainer;
struct HadoopContainer;
struct CarContainer;

impl Container for DockerContainer {
    fn describe() -> String {
        return String::from("Docker containers are deployed on top of the host OS kernel");
    }
}

impl Container for HadoopContainer {
    fn describe() -> String {
        return String::from("A Hadoop container is a unit of resource allocation on a single node, managed by YARN");
    }
}

impl Container for CarContainer {
    fn describe() -> String {
        return String::from("A car container contains cars and is moved on huge ships");
    }
}

impl DockerContainer {
    fn describe() -> String {
        return String::from("A Docker container contains code, runtime, sytem tools, libraries and configuration");
    }
}

fn identically_named_method() {
    let person = Human;
    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);


    // Containers
    let docker_container_raw_impl_description = DockerContainer::describe();
    println!("Docker container description prioritizes it default impl: {:?}", docker_container_raw_impl_description);
    let docker_container_container_impl_description = <DockerContainer as Container>::describe();
    println!("Docker container description and its Container impl: {:?}", docker_container_container_impl_description);
}

/* Supertrait */

use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {x: i32, y: i32}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "({}, {})", self.x, self.y);
    }
}
impl OutlinePrint for Point {}


fn supertrait_demo() {
    let p1 = Point {x: 102, y: 105};
    println!("The point p1 is {}", p1);
    p1.outline_print();

    let local_wrapper = LocalWrapper (vec![String::from("Hello"), String::from("Rust")]);
    println!("The local wrapper output is: {}", local_wrapper);
}

/*
YOUR CRATE                    STD CRATE (or another crate)
┌──────────────┐              ┌────────────────────────────┐
│              │              │  trait Display { ... }     │
│  struct Foo  │              │  struct Vec<T> { ... }     │
│              │              │  struct String             │
└──────────────┘              └────────────────────────────┘
       ▲                                   ▲
       │                                   │
       │  local to your crate              │  foreign to your crate
       └───────────────────────────────────┘

The orphan rule asks: for `impl Trait for Type`,
is `Trait` local?  is `Type` local?

┌─────────────────────────────┬──────────────────┬──────────────────┐
│  impl Display for Foo       │  Trait: foreign  │  Type:  local   │ ✅
│  impl MyTrait for Vec<T>    │  Trait: local    │  Type:  foreign │ ✅
│  impl Display for Vec<T>    │  Trait: foreign  │  Type:  foreign │ ❌
└─────────────────────────────┴──────────────────┴──────────────────┘

What if you really want that last line to work?
 */
struct LocalWrapper(Vec<String>);

impl fmt::Display for LocalWrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}