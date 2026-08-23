pub fn borrow_checking() {
/*
 * When an expression reads a variable, the variable’s value is copied from its slot in the stack
 frame.The value of a is copied into b , and a is left unchanged, even after changing b .
 */
 let a = 5;
 let mut b = a;
 b += 1;


 /*
  * Pointer lies on stack frame, pointee is on heap. This case only Box<T> (smart pointer). Smart pointer = stack, pointee = heap-only.
  * 
  * In this case, a has been moved permanently to b. We cannot use a anymore.
  */
  let a = Box::new([0;1_000_000]);
  let b = a;
  
}


/*
 * 1. "Minh" on heap. first (pointer) owns "Minh" 
 * 2. args equals to let. name (pointer) owns "Minh".
 * 3. Larger alloc -> write "Minh Junior" -> delete old heap memory -> first owns deallocated memory.
 * 4. Frame of add_suffix is gone, returned name. Ownership to full. 
 */
pub fn ownership_between() {
    let first = String::from("Minh");
    let full: String = add_suffix(first);
    println!("Full name of Minh is : {}", full);
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" .Junior");
    return name;
}