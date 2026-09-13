use std::fs;

pub fn read_poem(file_path: &str, query: &str) {
    println!("Reading {}", file_path);

    let contents =
        fs::read_to_string(file_path).unwrap_or_else(|e| panic!("Could not read '{file_path}': {e}"));

    for line in contents.lines().filter(|l| l.contains(query)) {
        println!("{line}");
    }
}