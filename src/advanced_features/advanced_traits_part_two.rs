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
}