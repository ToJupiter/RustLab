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
    let first_clone = first.clone();
    let full: String = add_suffix(first);
    println!("Full name of Minh is : {}", full);
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" .Junior");
    return name;
}

/*
 * Borrow m1, m2. &m1 is the reference to the String "hello"
 * 
 */
 pub fn borrow_not_owned() {
     let m1 = String::from("Hello");
     let m2 = String::from("world");
     greet(&m1, &m2);
     let s = format!("{} {}", m1, m2);
 }

 pub fn greet(g1: &String, g2: &String) {
     println!("{} {}", g1, g2);
 }


 /*
  * Deref a pointer accesses its data
  * &String: string ref of type String
  * 
  */
  pub fn deref_data (){
      let mut x: Box<i32> = Box::new(1);
      // a takes the value of "1" inside of heap
      let a: i32 = *x;
      // b ref -> x -> "1"
      let b = &x;
      // 

      let r1: &Box<i32> = &x;
      let r11: i32 = **r1;
      let mut r3: &Box<i32> = &x;
      // Error: cannot mut x because not "&mut" 
      // **r3 += 1;
      let mut r4: &mut Box<i32> = &mut x;
      **r4 += 1;

      // Borrow from x the "reference to heap data"
      let r2: &i32 = &*x;
      let d: i32 = *r2;
      
  }

  pub fn deref_exp() {
      let x1 : Box<i32> = Box::new(-1);
      let x2 = &x1;
      let x3 = &x2;
      let final_result = x3.abs();
      let compare_result = i32::abs(***x3);
      println!("Comparison: {} and {}", final_result, compare_result);
      assert_eq!(final_result, compare_result);
  }