use std::env;

use std::fs::File;
use std::io;
use std::io::Read;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        print_standard_error();
        return;
    }

    let mut available_gpios: HashSet<String> = HashSet::new();
    available_gpios.insert("gpio01".to_string());
    available_gpios.insert("gpio02".to_string());

    let gpio: &str = &args[1];
    let function: &str = &args[2];
    let mode: &str = &args[3];

    if !available_gpios.contains(gpio) {
        print_information("Invalid GPIO specified")
    }


}
/*
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
*/

fn read_gpio(gpio: &str) -> Result<String, io::Error> {
    let f = File::open(gpio);
    let mut f = match f {
        Ok(file) => file,
        Err(e)   => return Err(e)
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_)  => Ok(s),
        Err(e) => Err(e)
    }
}

fn print_information(message: &str) {
    println!("[BUSCTL] {}", message);
}

fn print_standard_error() {
    print_information("Specify at least 3 Arguments");
}