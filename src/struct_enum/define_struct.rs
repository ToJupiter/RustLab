/*
 * Struct: normal, tuple-like & unit
 * Follows ownership rules like other code
 * Used for traits
 */

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

pub struct RGBColor(u8, u8, u8);
pub struct AbstractStruct;

#[derive(Debug)]
pub struct Oxy(f64, f64);

impl Oxy {
    pub fn o_distance(&self) -> f64 {
        return f64::sqrt(self.0 * self.0 + self.1 * self.1);
    }

    pub fn is_further(&self, other_oxy: &Oxy) -> bool {
        let other_o_dist = other_oxy.o_distance();
        return self.o_distance() > other_o_dist && true;
    }
    
    pub fn x2_init(x: f64, y: f64) -> Self {
        return Self(2.0*x, 2.0*y);
    }

    pub fn set_x(&mut self, x: f64) {
        self.0 = x;
    }

    pub fn set_y(&mut self, y: f64) {
        self.1 = y;
    }
}

pub fn create_a_struct() -> User {
    let mut user1 = User {
        active: true,
        username: String::from("jupiter@example.com"),
        email: String::from("hehe@example.com"),
        sign_in_count: 1024
    };

    user1.email = String::from("tech@example.com");

    let mut a_color = RGBColor(255, 9, 20);
    let abstraction_struct = AbstractStruct;
    
    return user1;
}

pub fn short_hand_init(email: String, username: String) -> User {
    let mut user1 = create_a_struct();
    
    let mut user2 = User {
        active: true,
        sign_in_count: 512,
        username,
        email
    };

    let mut user3 = User {
        active: false,
        ..user1
    };

    // println!("User 1 username: {}", user1.username); Because String does not implement the Copy trait, we could not use user1.username after moving it to user3

    return user3;
}

pub fn print_oxy(p: &Oxy) {
    println!("The x and y coords are x: {} and y: {}", p.0, p.1);
}

pub fn oxy_ownership() {
    let mut new_oxy = Oxy(1.15, 10.05);

    let x_coord = &mut new_oxy.0;
    println!("The x coord of new_oxy is {}", x_coord);
    print_oxy(&new_oxy);

    println!("The y coord of new_oxy is {}", new_oxy.1);
    // println!("The x coord of new_oxy is {}", x_coord); the lifetime of x_coord ends here, causing it to not be printed.

    // In order to print it out, we must add the derive Debug.
    println!("The Oxy debug result is {:?}", new_oxy);
    println!("The distance to the center O is {}", new_oxy.o_distance());

    // Associated functions and life
    let x2_oxy = Oxy::x2_init(10.3, 9.5);
}

pub fn associated_and_method() {
    let new_oxy = Oxy(10.3, 9.5);

    let cmp1 = new_oxy.o_distance();
    let mut cmp2 = Oxy::o_distance(&new_oxy);
    assert_eq!(cmp1, cmp2);
}

