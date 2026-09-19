use crate::functional_pointer::rc_pointer::List::{Cons, Nil};
use crate::functional_pointer::rc_pointer::SampleLinkedList::{Cont, Zero};
use std::rc::Rc;

/*
 * Note that Rc<T> is only for use in single-threaded scenarios. When we discuss concurrency in Chapter 16, we’ll cover how to do reference counting in multithreaded programs.
 */
#[derive(Debug, Clone)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

/*
 * a -> 5 -> 10 -> Nil
 * b -> a
 */
pub fn rc_pointer_test() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    // let c = Cons(80, Box::new(a.clone())); -- if impl Clone trait that is bearable

    let a_list = Rc::new(Cont(50, Rc::new(Cont(100, Rc::new(Zero)))));
    println!("Number of refs to a is: {}", Rc::strong_count(&a_list));
    let b_list = Cont(100, Rc::clone(&a_list));
    let c_list = Cont(40, Rc::clone(&a_list));
    println!("Number of refs to a after creating c is: {}", Rc::strong_count(&a_list));
    {
        let d_list = Cont(500, Rc::clone(&a_list));
        println!("Count after creating d: {}", Rc::strong_count(&a_list));
    }
    println!("Count after d is out of scope: {}", Rc::strong_count(&a_list));
}

enum SampleLinkedList {
    Cont(i32, Rc<SampleLinkedList>),
    Zero
}

