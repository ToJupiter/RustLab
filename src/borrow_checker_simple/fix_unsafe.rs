use std::rc::Rc;

pub fn string_return() {
    let s1 = String::from("Hello from the Rust programming language");

    // return &s1; this will be unsafe if the return type is &String
    
    let flag: i32 = 1;
    // You can return full ownership to String

    if flag == 1 {
        // return s1; return the ownership of s1
    } else if flag == 2 {
        // return "Hello from the Rust programming language"
    } else if flag == 3 {
        let s2 = Rc::new(s1);
        // return Rc::clone(&s2); -> OK
    } else {
        let mut s2 = s1.clone();
        let mut s3 = &mut s2;
        s3.replace_range(.., "Hello world");
    }
}

pub fn add_big_strings_together(dst: &mut Vec<String>, str_list: &[String]) {
    let largest = dst.iter().max_by_key(|s| s.len()).unwrap();
    let largest_len = largest.len();
    // We end lifetime of largest early here. This returns the perms back to dst so we could do the push operation

    for stri in str_list {
        if stri.len() > largest_len {
            dst.push(stri.clone());
        }
    }
}

pub fn array_element() {
    let mut arr = [0,1,2,3];
    let (a_l, a_r) = arr.split_at_mut(2);
    let x = &mut a_l[1];
    let y = &a_r[0];
    *x += *y;
}

pub fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

pub fn string_slice() {
    let s = String::from("Hello world");

    // The slices are fat pointers: {"ptr": to_slice, "len": 5}
    let hello = &s[0..6];
    let world = &s[7..];
    let s2 = &s;
}

pub fn first_word_as_bytes(s: &String) {
    let bytes = s.as_bytes();
    println!("The bytestring is: {:?}", bytes);

    let string_lit: &str = "Hello rust";
    println!(
      "&String={} &str={}",
      std::mem::size_of::<&String>(),
      std::mem::size_of::<&str>(),
    );
}

type Document = Vec<String>;

fn new_document(words: Vec<String>) -> Document {
    return words;
}

fn add_word(this: &mut Document, word: String) {
    this.push(word);
}

fn get_words(this: &Document) -> &[String] {
    this.as_slice()
}

pub fn type_conversion() {
    let words = vec!["hello".to_string()];
    let d = new_document(words);

    // .to_vec() converts &[String] to Vec<String> by cloning each string
    let words_copy = get_words(&d).to_vec();
    let mut d2 = new_document(words_copy);
    add_word(&mut d2, "world".to_string());

    assert!(!get_words(&d).contains(&"world".into()));
}

