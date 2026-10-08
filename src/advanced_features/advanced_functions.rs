use std::ffi::c_void;
use std::mem::size_of;


fn add_one(x: i32) -> i32 {
    return x + 1;
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    return f(arg) + f(arg);
}

pub fn advanced_functions_and_closures() {
    println!("For testing purpose only: {}", do_twice(add_one, 50));

    let mut vector_hello = [5, 2, 8, 1, 9, 3];
    unsafe {
        qsort (
            vector_hello.as_mut_ptr() as *mut c_void,
            vector_hello.len(),
            size_of::<i32>(),
            cmp_i32_asc
        );
    }
    println!("Asc sort: {:?}", vector_hello);

    unsafe {
        qsort (
            vector_hello.as_mut_ptr() as *mut c_void,
            vector_hello.len(),
            size_of::<i32>(),
            cmp_i32_desc
        );
    }
    println!("Desc sort: {:?}", vector_hello);
    
}

/* 
    Using FFI in order to demonstrate how function passing works:
    ┌──────────────────────────────────────────────────────────────┐
    │                                                              │
    │   C sees:  a function pointer                                │
    │              └── code address + C calling convention         │
    │                                                              │
    │   Rust can produce this with:  extern "C" fn foo(...) { }    │
    │                                                              │
    │   Rust CANNOT produce this with:                             │
    │       • a capturing closure  (extra state, no place to put it)│
    │       • a non-capturing closure (has Rust ABI, not C ABI)     │
    │                                                              │
    └──────────────────────────────────────────────────────────────┘
 */

// qsort from libc — the C signature is:
//   void qsort(void *base, size_t nmemb, size_t size,
//              int (*compar)(const void *, const void *));
unsafe extern "C" {
    fn qsort(
        base: *mut c_void,
        nmemb: usize,
        size: usize,
        compar: extern "C" fn(*const c_void, *const c_void) -> i32
    );
}

// A plain function with C ABI. No captures possible — it's not a closure.
extern "C" fn cmp_i32_asc(a: *const c_void, b: *const c_void) -> i32 {
    // SAFETY: qsort passes pointers to elements of the array we handed it,
    // so each pointer is valid for reading an i32.
    let a = unsafe { *(a as *const i32) };
    let b = unsafe { *(b as *const i32) };
    if a < b { -1 } else if a > b { 1 } else { 0 }
}

extern "C" fn cmp_i32_desc(a: *const c_void, b: *const c_void) -> i32 {
    cmp_i32_asc(b, a)
}


