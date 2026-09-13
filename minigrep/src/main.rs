use std::env;

use minigrep::file_read::read_poem;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <query> <file_path>", args[0]);
        std::process::exit(1);
    }
    let query = &args[1];
    let file_path = &args[2];
    println!("We are searching for: {}", query);
    println!("We are looking in file: {}", file_path);

    read_poem(file_path, query);
}
