/*
 * GPIOCTL 
 * Provides functions to interact with the GPIOs of the BeagleBoneBlack
 */

mod gpio;

use std::env;
use std::fs::File;
use std::fs;
use std::io::Read;
use std::path::Path;

/*
const GPIO_PATH: &str           = "/sys/class/gpio";
const GPIO_EXPORT_PATH: &str    = "/sys/class/gpio/export";
const GPIO_UNEXPORT_PATH: &str  = "/sys/class/gpio/unexport";
*/
const GPIO_PATH: &str           = "/home/buuhuu/dev/rust/bbb/bbb-gpioctl/gpio";
const GPIO_EXPORT_PATH: &str    = "/home/buuhuu/dev/rust/bbb/bbb-gpioctl/gpio/export";
//const GPIO_UNEXPORT_PATH: &str  = "/home/buuhuu/dev/rust/bbb/bbb-gpioctl/gpio/unexport";


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        print_standard_error();
        return;
    }

    let available_gpios = gpio::get_system_gpios();
    let mut gpio_exported: bool = false;

    let gpio: &str = &args[1];
    let mode: &str = &args[2];
    let function: &str = &args[3];
    let mut set_expression: &str = "";
    if args.len() > 4 {
        set_expression = &args[4];
    }

    let mut available_modes_gpio: Vec<&gpio::Mode> = vec![];
    
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

    if !Path::new(&format!("{}/{}", GPIO_PATH, gpio)).exists() {
        export_gpio(gpio);
        gpio_exported = true;
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
            match set_expression.parse::<i32>() {
                Ok(n) => {
                    if n == 0 || n == 1 {
                        set_value(gpio, n)
                    } else {
                        print_information("Value must be 0 or 1");
                        return;
                    }
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

fn export_gpio(gpio: &str) {
    let path: String = String::from(GPIO_EXPORT_PATH);
    if !Path::new(&path).exists() {
        print_information(&format!("Can't find export file. Looking for: {}", GPIO_EXPORT_PATH));
        return;
    }
    fs::write(&path, gpio.to_string()).expect(&format!("Can't write '{}' to export file", gpio));
    print_information(&format!("Exported {}", gpio));
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
    let res = res.trim();
    String::from(res)
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
    let res = res.trim();
    String::from(res)
}

fn set_value(gpio: &str, value: i32) {
    // First, check, if direction is "out"
    if get_direction(gpio) == "in" {
        print_information("Can't write value when direction is [IN]");
        return;
    }
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
    let res = res.trim();
    String::from(res)
}

fn print_information(message: &str) {
    println!("[BUSCTL] {}", message);
}

fn print_standard_error() {
    print_information("Specify at least 3 Arguments");
}