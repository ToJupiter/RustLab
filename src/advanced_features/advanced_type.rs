use std::fmt;
use std::io::Error;
use std::collections::HashMap;

fn hard_type() {
    type HardType = Box<dyn Fn() + Send + 'static>;

    let f: HardType = Box::new(|| println!("Hello"));


    // Cannot type-mix them
    let m = Meters(5);
    let mm = Millimeters(500);
    
    add_millimeters(&mm, &mm);   // ✅
    // add_millimeters(m, mm);    // ❌ compile error: expected Millimeters, found Meters
}

/*
┌───────────────┐        ┌───────────────┐
│ Millimeters   │        │ Meters        │
│ ┌───────────┐ │        │ ┌───────────┐ │
│ │ u32: 500  │ │        │ │ u32: 5    │ │
│ └───────────┘ │        │ └───────────┘ │
└───────────────┘        └───────────────┘
     ▲                        ▲
     │ same bytes, DIFFERENT types │
     │ compiler will not let you mix them
     └────────────────────────┘
 */

#[derive(Debug)]
struct Millimeters(u32);
struct Meters(u32);

fn add_millimeters(a: &Millimeters, b: &Millimeters) -> Millimeters {
    println!("The output is: {:?}", Millimeters(a.0 + b.0));
    Millimeters(a.0 + b.0)
}

/*
┌───────────────────────────────────────────────┐
│              Newtype Pattern                  │
├───────────────────────┬───────────────────────┤
│   Type identity       │   Encapsulation       │
│   (Meters ≠ Feet)     │   (People hides Map)  │
│                       │                       │
│   Prevents mixing     │   Prevents coupling   │
│   unrelated values    │   to representation   │
└───────────────────────┴───────────────────────┘
 */
pub struct People {
    inner: HashMap<i32, String>,   // private field
}

impl People {
    pub fn new() -> Self { People { inner: HashMap::new() } }

    pub fn add(&mut self, name: String) {
        let id = self.inner.len() as i32 + 1;
        self.inner.insert(id, name);
    }

    pub fn name_of(&self, id: i32) -> Option<&String> {
        self.inner.get(&id)
    }
}


/*
Common expressions with type !:
1. panic!(...)
2. continue
3. break (inside the loop it breaks)
4. return (in a function returning something else)
5. an infinite loop { } with no break
6. std::process::exit(...)

arm 1: Ok(num) => num        type: u32
arm 2: Err(_) => continue    type: !  ← coerces to u32
─────────────────────────────────────────
overall match type:          u32
*/
fn bar() -> ! {
    std::process::exit(0)
}

/*
A DST must always live behind a pointer of some kind.

Valid:
1, &str, &mut str
2, Box<str>, Rc<str>, Arc<str>
3, &dyn Trait, Box<dyn Trait>
4,&[T], Box<[T]>

Invalid:
1. str alone
2. dyn Trait alone
3. [T] alone
 */