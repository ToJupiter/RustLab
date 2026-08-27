# Notes:

## Ownership
1. We have learnt about Box<T> smart pointer in Rust: pointer lies inside of stack frame, Box creates a new pointee inside of heap. 
2. Ownership transfer:
``` rs
let a = Box::new(5);
let b = a;
// Ownership of the "5" inside of heap transfered to b. When free is called, it only cleans the "5" once (when b is cleaned).
```

## Borrowing
1. Aliasing and mutation should not happen at the same time.
2. **Pointer Safety Principle**: data should never be aliased and mutated at the same time.
