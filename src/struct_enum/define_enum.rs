/* First way of IP definition */
enum IPAddrKind {
    V4,
    V6
}

pub struct ip_addr {
    kind: IPAddrKind,
    address: String
}

/* Directly integrate dtypes */
enum IPAddrIntegrated {
    V4(u8, u8, u8, u8),
    V6(String)
}

/* Use struct inside of enum def */
pub struct IPV4Inside {
    first: u8,
    second: u8,
    third: u8,
    fourth: u8
}

pub struct IPV6Inside {
    inner: String
}

pub enum IPAddressWrap {
    V4(IPV4Inside),
    V6(IPV6Inside)
}

/* Final type of enum */
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

pub fn create_ip() {
    let example_ipv4 = ip_addr {kind: IPAddrKind::V4, address: String::from("192.168.1.1")};
    
}

pub fn option_type() {
    /*
     * Option's purpose is to prevent us from assigning null to non-null value
     * enum Option<T> {Some(T), None}
     */
     let small_option: Option<i8> = Some(5);
     let small_converted = Option::unwrap_or_default(small_option);
     let five: i8 = 5;

     // assert_eq!(small_option, five); Cannot run because different type
     assert_eq!(small_converted, five);
}

/* US coins enum */
#[derive(Debug)]
enum USStates {
    Arizona,
    Alabama,
    Washington,
    California,
    Texas
}

enum USCoins {
    Penny,
    Nickel,
    Dime,
    Quarter(USStates),
}

pub fn us_coins_matching_arm(coin: &USCoins) -> u8 {
    match coin {
        USCoins::Penny => 1,
        USCoins::Nickel => 5,
        USCoins::Dime => 10,
        USCoins::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

pub fn vending_quarter_machine(coin: &USCoins) {
    match coin {
        USCoins::Quarter(state) => println!("Took in the quarter"),
        _ => ()
    }
}

pub fn us_coins() {
    let arizona_coin = USCoins::Quarter(USStates::Arizona);
    us_coins_matching_arm(&arizona_coin);
    vending_quarter_machine(&arizona_coin);
}

/* Hard example with match keyword */
pub fn print_optional(option: Option<String>) {
    match option {
        Some(_) => println!("Some!"),
        None => println!("None")
    }
}

pub fn print_optional_owned(option: Option<String>) {
    match option {
        Some(s) => println!("Some {}", s),
        None => println!("None")
    }
}

pub fn print_optional_borrowed(option: &Option<String>) {
    match option {
        Some(s) => println!("Some {}", s),
        None => println!("None")
    }
}

pub fn option_string_print() {
    let optional_string: Option<String> = Some(String::from("Hello Rust lang"));

    // Only borrow the string inside
    print_optional_borrowed(&optional_string);

    // Both owned the optional_string
    print_optional(optional_string);
    // print_optional_owned(optional_string);
}

/*
 * If let matching, not having to cover all cases
 */

 pub fn optional_if_let() {
     let an_integer : Option<i32> = Some(1024);

     if let Some(1024) = an_integer {
         println!("Two some equal!");
     }
 }