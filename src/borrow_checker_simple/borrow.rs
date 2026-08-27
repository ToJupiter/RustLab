use std::rc::Rc;

/*
 * Read (R): data can be copied to another location.
 * Write (W): data can be mutated.
 * Own (O): data can be moved or dropped.
 * 
 * Second, why do places lose permissions when they become unused? Because some permissions
 are mutually exclusive. If you write num = &v[2] , then v cannot be mutated or dropped while num is in use. But that doesn’t mean it’s invalid to use num again. For example, if we add another println! to the above program, then num simply loses its permissions one line later
 */
 pub fn simple_ref_arr() {

     // v has all 3 RWO 
     let mut v: Vec<i32> = vec![1,2,3];

     // the data in v: borrowed by num. 
     // WO is lost from v (no mut, borrowed = not owned). 
     // num: gets RO.
     // *num: read-only
     let num: &i32 = &v[2];

     // num no longer in use, v is not borrowed anymore. v gained WO. num, *num losts all perms.
     println!("The third element of the arr: {}", *num);
     println!("Once again, the third element of the arr: {}", *num);
     

     // v no longer in use, lost all perms.
     v.push(4);
 }

 /*
  * More generally, permissions are defined on places and not just variables. A place is anything you
  can put on the left-hand side of an assignment. Places include:
  Variables, like a .
  Dereferences of places, like *a .
  Array accesses of places, like a[0] .
  Fields of places, like a.0 for tuples or a.field for structs (discussed next chapter).
  Any combination of the above, like *((*a)[0].1) 
  */
 pub fn compare_perms() {

     // x is i32, on heap. x: RO, no W.
     let x = 0;

     // x_ref: RWO. *x_ref does not have W
     let mut x_ref = &x;
     // *x_ref += 1; -> Error

     let mut x2 = 0;
     let mut x2_ref = &x2;
     // *x2_ref += 1; if the reference is immutable then code is invalid
     // 

     // This code is correct, and does not even need x3_ref to be mutable.
     // De-ref it: *x3_ref - mutable.
     let mut x3 = 0;
     let x3_ref = &mut x3;
     *x3_ref += 1;
 }

 pub fn vector_borrow() {
     let mut v = vec![1,2,3];

     let p1 = &v[2];
     let p2 = &v[1];

     // v.push(4); cannot borrow v as mutable because it is also borrowed as immutable
     // mutable borrow occurs here (rustc E0502)

     println!("position 2 value is {p1}");
     //   pub fn push(&mut self, value: T) {
     //     let _ = self.push_mut(value);
     // }
     // push: &mut self
     v.push(4);
}

/*
 * When num was an immutable reference, v still had the R permission. Now that num is a mutable reference, v has lost all permissions while num is in use.
 * 
 */
pub fn mutable_ref () {
    let mut v: Vec<i32> = vec![1,2,3];

    let num: &mut i32 = &mut v[2];
    *num += 1;

    let num2: &mut i32 = &mut *num;
    *num2 += 1;
       
    println!("Third element is {}", *num);
    // println!("Third element is {num2}"); code works above because of NLL and Reborrowing
    println!("Vector is now {:?}", v);
}

pub fn mutable_borrowing() {
    let mut v: Vec<u32> = vec![3,10,15];

    let element_2 = &mut v[1];
    // let element_3 = &mut v[2]; this caused error imm - nothing can be mutable borrowing twice

    *element_2 += 1;
    // *element_3 += 1; 
    
}


/*
 * Data must outlive all of its reference. In a scope, Rust knows the lifetime of a variable (inside a function that could be checked).
 * Shortened rule: no other permission should outlive O. The moment O is destroyed, everything else is gone
 * 
 */

pub fn destroyed_or_own_string() -> String {
    let s = String::from("Hello the Rust language");

    let s1 = &s;
    
    // return s1; -- even if the return type is &String, this will be wrong because it will make the Read perm -> stay longer than Own perm.

    let s2 = s;

    // Returning the whole "Own" permission is okay, satisfying the lifetime of the function
    return s2;
}

pub fn string_literal_return() -> &'static str {
    return "Hello Rust language";
}

pub fn rc_string_return() -> Rc<String> {
    let s = Rc::new(String::from("Hello from Rust lang"));
    let rc_clone = Rc::clone(&s);
    return rc_clone;
}

pub fn caller_create_string_return(output: &mut String) {
    return output.replace_range(.., "Hello from Rust lang");
}