use std::env;

fn main() {
    println!("\n");

    if env::args().len() != 2 {
        panic!("Error(00): invalid parameters ...");
    }

    let commands: Vec<String> = env::args().collect();

    let (temp_unit, temp) = check_input_argument(&commands[1]);
    match temp_unit {
        TempratureUnit::Celsius => {
            let result = celsius_to_fahrenheit(temp);
            println!(" -> Celsius: {temp:.2} = Fahrenheit: {result:.2}");
        }
        TempratureUnit::Fahrenheit => {
            let result = fahrenheit_to_celsius(temp);
            println!(" -> Fahrenheit: {temp:.2} = Celsius: {result:.2}");
        }
    }

    println!("\n The End ... (0.0.4)\n");
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0_f64) / 1.8_f64
}

fn celsius_to_fahrenheit(f: f64) -> f64 {
    (f * 1.8_f64) + 32.0_f64
}

fn check_input_argument(arg: &str) -> (TempratureUnit, f64) {
    let mut spl = arg.split(":");

    let tmp_unt: TempratureUnit = match spl.next() {
        Some(temp_unit) => {
            if temp_unit.eq_ignore_ascii_case("f") {
                TempratureUnit::Fahrenheit
            } else if temp_unit.eq_ignore_ascii_case("c") {
                TempratureUnit::Celsius
            } else {
                panic!("Error(01): invalid parameters ...");
            }
        }
        None => panic!("Error(02): invalid parameters ..."),
    };

    match spl.next() {
        Some(str_num) => {
            let temprature = str_num
                .trim()
                .parse()
                .expect("Error(05): invalid temprature number ...");
            (tmp_unt, temprature)
        }
        None => panic!("Error(03): invalid parameters ..."),
    }
}

enum TempratureUnit {
    Celsius,
    Fahrenheit,
}
