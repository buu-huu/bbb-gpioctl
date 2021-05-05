use std::fmt;

pub fn create_gpios() -> Vec<Gpio> {
    let mut available_gpios: Vec<Gpio> = vec![];
    available_gpios.push(
        Gpio::new(String::from("gpio66"), 1, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio69"), 2, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios
}

pub struct Gpio {
    pub name: String,
    pub number: i32,
    pub modes: Vec<Mode>,
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

pub enum Mode {
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