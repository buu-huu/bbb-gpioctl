use std::env;

use std::fmt;
use std::fs::File;
use std::io;
use std::io::Read;
use std::collections::HashSet;

const GPIO_PATH: &str = "/home/buuhuu/dev/rust/bbb/gpioctl/";

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
    let mode: &str = &args[2];
    let function: &str = &args[3];
    let mut set_expression: &str = "";
    if args.len() > 4 {
        set_expression = &args[4];
    }

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
    
    // Check if specified GPIO is available and get modes
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
    
    if function != "get" && function != "set" {
        print_information("Invalid Function");
        return;
    }

    let res: String;
    if function == "get" {
        if mode == "direction" {
            res = get_direction(gpio);
        } else if mode == "value" {
            res = get_value(gpio);
        } else if mode == "label" {
            res = get_label(gpio);
        } else {
            print_information("Please specify a correct mode");
            return;
        }
    } else if function == "set" {
        if mode == "direction" {
            if set_expression != "in" && set_expression != "out" {
                print_information("Please use [in] or [out]");
                return;
            }
            set_direction(gpio, set_expression);
        }
        else if mode == "value" {
            match set_expression.parse::<i32>() {
                Ok(n) => set_value(gpio, n),
                Err(e) => {
                    print_information("Please specify a number");
                    return;
                }
            }
        } else {
            print_information("Please specify correct mode");
            return;
        }
    } else {
        print_information("Wrong function");
    }


}

fn read_gpio(gpio: &str) -> Result<String, io::Error> {
    let path: String = format!("{}{}", GPIO_PATH, gpio);
    let f = File::open(path);
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

fn get_direction(gpio: &str) -> String {
    println!("Called set_direction for GPIO {}", gpio);
    String::new()
}

fn set_direction(gpio: &str, direction: &str) {
    println!("Called set_direction for GPIO {}, Direction: {}", gpio, direction);
}

fn get_value(gpio: &str) -> String {
    println!("Called get_value for GPIO {}", gpio);
    String::new()
}

fn set_value(gpio: &str, value: i32) {
    println!("Called set_value for GPIO {}", gpio);
}

fn get_label(gpio: &str) -> String {
    println!("Called get_label for GPIO {}", gpio);
    String::new()
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
        gpio
    }
}