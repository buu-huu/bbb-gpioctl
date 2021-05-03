/*
 * GPIOCTL 
 * Provides functions to interact with the GPIOs of the BeagleBoneBlack
 * Example call: gpioctl gpio01 [direction|value|label] [get|set]
 */

use std::env;

use std::fmt;
use std::fs::File;
use std::fs;
use std::io::Read;
use std::path::Path;

const GPIO_PATH: &str = "/home/buuhuu/dev/rust/bbb/gpioctl";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        print_standard_error();
        return;
    }

    // Todo: Create Parser for available GPIOs from JSON file
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
            println!("{}", res);
        } else if mode == "value" {
            res = get_value(gpio);
            println!("{}", res);
        } else if mode == "label" {
            res = get_label(gpio);
            println!("{}", res);
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
            match set_expression.parse::<f32>() {
                Ok(n) => {
                    if n > 5.0 || n < 0.0 {
                        print_information("Value must be between 0 and 5");
                        return;
                    }
                    set_value(gpio, n)
                },
                Err(_) => {
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

fn read_file(path: String) -> String {
    let f = File::open(path);
    let mut f = match f {
        Ok(file) => file,
        Err(_)   => panic!("Error reading GPIO file"),
    };

    let mut s = String::new();
    f.read_to_string(&mut s).expect("Error reading GPIO file");
    s
}

fn get_direction(gpio: &str) -> String {
    let path: String = format!("{}/{}/{}", GPIO_PATH, gpio, "direction");
    let res = read_file(path);
    res
}

fn set_direction(gpio: &str, direction: &str) {
    let path: String = format!("{}/{}/{}", GPIO_PATH, gpio, "direction");
    if !Path::new(&path).exists() {
        print_information("Can't write to GPIO file. Direction file does not exist");
        return;
    }
    fs::write(&path, direction).expect("Error writing GPIO file");
}

fn get_value(gpio: &str) -> String {
    let path: String = format!("{}/{}/{}", GPIO_PATH, gpio, "value");
    let res = read_file(path);
    res
}

fn set_value(gpio: &str, value: f32) {
    let path: String = format!("{}/{}/{}", GPIO_PATH, gpio, "value");
    if !Path::new(&path).exists() {
        print_information("Can't write to GPIO file. Value file does not exist");
        return;
    }
    fs::write(&path, value.to_string()).expect("Error writing GPIO file");
}

fn get_label(gpio: &str) -> String {
    let path: String = format!("{}/{}/{}", GPIO_PATH, gpio, "label");
    let res = read_file(path);
    res
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
    Label,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           Mode::Direction => write!(f, "Direction"),
           Mode::Value     => write!(f, "Value"),
           Mode::Label     => write!(f, "Label"),
       }
    }
}

struct Gpio {
    name: String,
    number: i32,
    modes: Vec<Mode>,
}

impl Gpio {
    fn new(name: String, number: i32, modes: Vec<Mode>) -> Gpio {
        let gpio: Gpio = Gpio {
            name: name,
            number: number,
            modes: modes,
        };
        gpio
    }
}