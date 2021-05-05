use std::fmt;

pub fn get_system_gpios() -> Vec<Gpio> {
    let mut available_gpios = vec![];

    available_gpios.push(
        Gpio::new(String::from("gpio0"), 0, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio2"), 2, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio3"), 3, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio4"), 4, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio5"), 5, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio7"), 7, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio8"), 8, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio9"), 9, vec![Mode::Direction, Mode::Value, Mode::Label])
    );


    available_gpios.push(
        Gpio::new(String::from("gpio10"), 10, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio11"), 11, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio12"), 12, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio13"), 13, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio14"), 14, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio15"), 15, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio19"), 19, vec![Mode::Direction, Mode::Value, Mode::Label])
    );


    available_gpios.push(
        Gpio::new(String::from("gpio20"), 20, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio22"), 22, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio23"), 23, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio26"), 26, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio27"), 27, vec![Mode::Direction, Mode::Value, Mode::Label])
    );


    available_gpios.push(
        Gpio::new(String::from("gpio30"), 30, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio31"), 31, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio32"), 32, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio33"), 33, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio34"), 34, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio35"), 35, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio36"), 36, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio37"), 37, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio38"), 38, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio39"), 39, vec![Mode::Direction, Mode::Value, Mode::Label])
    );


    available_gpios.push(
        Gpio::new(String::from("gpio44"), 44, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio45"), 45, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio46"), 46, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio47"), 47, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio48"), 48, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio49"), 49, vec![Mode::Direction, Mode::Value, Mode::Label])
    );


    available_gpios.push(
        Gpio::new(String::from("gpio50"), 50, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio51"), 51, vec![Mode::Direction, Mode::Value, Mode::Label])
    );


    available_gpios.push(
        Gpio::new(String::from("gpio60"), 60, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio61"), 61, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio62"), 62, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio63"), 63, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio65"), 65, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio66"), 66, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio67"), 67, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio68"), 68, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio69"), 69, vec![Mode::Direction, Mode::Value, Mode::Label])
    );


    available_gpios.push(
        Gpio::new(String::from("gpio70"), 700, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio71"), 71, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio72"), 72, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio73"), 73, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio74"), 74, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio75"), 75, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio76"), 76, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio77"), 77, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio78"), 78, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio79"), 79, vec![Mode::Direction, Mode::Value, Mode::Label])
    );


    available_gpios.push(
        Gpio::new(String::from("gpio80"), 80, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio81"), 81, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio86"), 86, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio87"), 87, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio88"), 88, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio89"), 89, vec![Mode::Direction, Mode::Value, Mode::Label])
    );


    available_gpios.push(
        Gpio::new(String::from("gpio110"), 110, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio111"), 111, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio112"), 112, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio113"), 113, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio114"), 114, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio115"), 115, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio116"), 116, vec![Mode::Direction, Mode::Value, Mode::Label])
    );
    available_gpios.push(
        Gpio::new(String::from("gpio117"), 117, vec![Mode::Direction, Mode::Value, Mode::Label])
    );

    available_gpios
}

#[derive(Clone)]
pub struct Gpio {
    pub name: String,
    pub number: i32,
    pub modes: Vec<Mode>,
}

impl Gpio {
    pub fn new(name: String, number: i32, modes: Vec<Mode>) -> Gpio {
        let gpio: Gpio = Gpio {
            name: name,
            number: number,
            modes: modes,
        };
        gpio
    }
}

#[derive(Clone)]
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