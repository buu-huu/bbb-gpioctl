use std::env;

use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::io::Read;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("Specify at least 3 Arguments");
        return;
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


enum AvailableGpios {
    gpio01,
    gpio02
}