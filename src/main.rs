use std::env;

use std::fmt;
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

    let mut available_gpios: Vec<Gpio> = vec![];
    available_gpios.push(
        Gpio::new(String::from("gpio01"), 1, vec![Mode::Direction, Mode::Value])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio02"), 2, vec![Mode::Direction, Mode::Value, Mode::Label])
    );

    let gpio: &str = &args[1];
    let function: &str = &args[2];
    let mode: &str = &args[3];

    /*
    let mode = match function {
        "direction" => Mode::Direction,
        "value"     => Mode::Value,
        "label"     => Mode::Label,
        _           => {
            print_information("Wrong method");
            return;
        }
    };
    */

    let mut available_modes_gpio: Vec<&Mode> = vec![];
    
    // Check if specified GPIO is available
    let gpio_iter = available_gpios.iter();
    let mut gpio_found = false;
    for val in gpio_iter {
        if gpio == val.name {
            gpio_found = true;
            let mode_iter = val.modes.iter();
            for mode in mode_iter {
                available_modes_gpio.push(mode);
            }
            break;
        }
    }
    if !gpio_found {
        print_information("Invalid GPIO");
        return;
    }

    println!("{}", available_modes_gpio[2]);



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

enum Mode {
    Direction,
    Value,
    Label
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           Mode::Direction => write!(f, "Direction"),
           Mode::Value     => write!(f, "Value"),
           Mode::Label     => write!(f, "Label")
       }
    }
}

struct Gpio {
    name: String,
    number: i32,
    modes: Vec<Mode>
}

impl Gpio {
    fn new(name: String, number: i32, modes: Vec<Mode>) -> Gpio {
        let gpio: Gpio = Gpio {
            name: name,
            number: number,
            modes: modes
        };
        return gpio;
    }
}