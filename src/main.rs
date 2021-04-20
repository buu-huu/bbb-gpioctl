use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let inhalt = read_file("gpio02");
    println!("{}", inhalt);
}

fn read_file(path: &str) -> String {
    let path = Path::new(path);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    file.read_to_string(&mut s).expect("Error reading file!");
    return s;
}