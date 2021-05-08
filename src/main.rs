/* *****************************************************************************
 *
 * GPIOCTL 
 * Provides functions to interact with the GPIOs of the BeagleBoneBlack
 * 
 * ****************************************************************************/

mod gpio;

use std::env;
use std::fs::File;
use std::fs;
use std::io::Read;
use std::path::Path;

const GPIO_PATH: &str           = "/sys/class/gpio";
const GPIO_EXPORT_PATH: &str    = "/sys/class/gpio/export";
const GPIO_UNEXPORT_PATH: &str  = "/sys/class/gpio/unexport";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        print_standard_error();
        return;
    }

    let available_gpios = gpio::get_system_gpios();

    let gpio_name: &str = &args[1];
    let mode: &str = &args[2];
    let function: &str = &args[3];
    let mut set_expression: &str = "";
    if args.len() > 4 {
        set_expression = &args[4];
    }

    // Fill with dummy values, initializing below with actual values if gpio is available/correct
    let mut gpio_selected: gpio::Gpio = gpio::Gpio {
        name: String::from("none"),
        number: 9999,
        modes: vec![],
    };
    
    // Check if specified GPIO is available and get modes
    let gpio_iter = available_gpios.iter();
    let mut gpio_found = false;
    for val in gpio_iter {
        if gpio_name == val.name {
            gpio_selected = val.clone();
            gpio_found = true;
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
            res = get_direction(gpio_selected.number, &gpio_selected.name);
            print_result(&res);
        } else if mode == "value" {
            res = get_value(gpio_selected.number, &gpio_selected.name);
            print_result(&res);
        } else if mode == "label" {
            res = get_label(gpio_selected.number, &gpio_selected.name);
            print_result(&res);
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
            set_direction(gpio_selected.number, &gpio_selected.name, set_expression);
        }
        else if mode == "value" {
            match set_expression.parse::<i32>() {
                Ok(n) => {
                    if n == 0 || n == 1 {
                        set_value(gpio_selected.number, &gpio_selected.name, n);
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

fn export_gpio(gpio_number: i32) {
    let path: String = String::from(GPIO_EXPORT_PATH);
    if !Path::new(&path).exists() {
        print_information(&format!("Can't find export file. Looking for: {}", GPIO_EXPORT_PATH));
        return;
    }
    fs::write(&path, gpio_number.to_string()).expect(&format!("Can't write '{}' to export file", gpio_number));
    print_information(&format!("Exported GPIO number {}", gpio_number));
}

fn unexport_gpio(gpio_number: i32) {
    let path: String = String::from(GPIO_UNEXPORT_PATH);
    if !Path::new(&path).exists() {
        print_information(&format!("Can't find unexport file. Looking for: {}", GPIO_EXPORT_PATH));
        return;
    }
    fs::write(&path, gpio_number.to_string()).expect(&format!("Can't write '{}' to unexport file", gpio_number));
    print_information(&format!("Unexported GPIO number {}", gpio_number));
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

fn get_direction(gpio_number: i32, gpio: &str) -> String {
    let mut exported: bool = false;
    if !Path::new(&format!("{}/{}", GPIO_PATH, gpio)).exists() {    // If GPIO folder already exists, don't
        export_gpio(gpio_number);                                   // export GPIO
        exported = true;
    }
    let path: String = format!("{}/{}/{}", GPIO_PATH, gpio, "direction");
    let res = read_file(path);
    let res = res.trim();

    if exported {                                                   // Only unexport when we exported first
        unexport_gpio(gpio_number);
    }
    String::from(res)
}

fn set_direction(gpio_number: i32, gpio: &str, direction: &str) {
    let mut exported: bool = false;
    if !Path::new(&format!("{}/{}", GPIO_PATH, gpio)).exists() {    // If GPIO folder already exists, don't
        export_gpio(gpio_number);                                   // export GPIO
        exported = true;
    }
    let path: String = format!("{}/{}/{}", GPIO_PATH, gpio, "direction");
    if !Path::new(&path).exists() {
        print_information("Can't write to GPIO file. Direction file does not exist");
        return;
    }
    fs::write(&path, direction).expect("Error writing GPIO file");

    if exported {                                                   // Only unexport when we exported first
        unexport_gpio(gpio_number);
    }
    print_information("Direction set");
}

fn get_value(gpio_number: i32, gpio: &str) -> String {
    let mut exported: bool = false;
    if !Path::new(&format!("{}/{}", GPIO_PATH, gpio)).exists() {    // If GPIO folder already exists, don't
        export_gpio(gpio_number);                                   // export GPIO
        exported = true;
    }
    let path: String = format!("{}/{}/{}", GPIO_PATH, gpio, "value");
    let res = read_file(path);
    let res = res.trim();

    if exported {                                                   // Only unexport when we exported first
        unexport_gpio(gpio_number);
    }
    String::from(res)
}

fn set_value(gpio_number: i32, gpio: &str, value: i32) {
    let mut exported: bool = false;
    if !Path::new(&format!("{}/{}", GPIO_PATH, gpio)).exists() {    // If GPIO folder already exists, don't
        export_gpio(gpio_number);                                   // export GPIO
        exported = true;
    }
    // First, check, if direction is "out"
    if get_direction(gpio_number, gpio) == "in" {
        print_information("Can't write value when direction is [IN]");
        return;
    }
    let path: String = format!("{}/{}/{}", GPIO_PATH, gpio, "value");
    if !Path::new(&path).exists() {
        print_information("Can't write to GPIO file. Value file does not exist");
        return;
    }
    fs::write(&path, value.to_string()).expect("Error writing GPIO file");

    if exported {                                                   // Only unexport when we exported first
        unexport_gpio(gpio_number);
    }
    print_information("Value set");
}

fn get_label(gpio_number: i32, gpio: &str) -> String {
    let mut exported: bool = false;
    if !Path::new(&format!("{}/{}", GPIO_PATH, gpio)).exists() {    // If GPIO folder already exists, don't
        export_gpio(gpio_number);                                   // export GPIO
        exported = true;
    }
    let path: String = format!("{}/{}/{}", GPIO_PATH, gpio, "label");
    let res = read_file(path);
    let res = res.trim();

    if exported {                                                   // Only unexport when we exported first
        unexport_gpio(gpio_number);
    }
    String::from(res)
}

fn print_information(message: &str) {
    println!("[BUSCTL] {}", message);
}

fn print_result(message: &str) {
    println!("[BUSCTL] Result: {}", message);
}

fn print_standard_error() {
    print_information("Specify at least 3 Arguments");
}