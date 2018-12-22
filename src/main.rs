use std::io;

const CONVERTION_RATIO :f64 = 9f64/5f64;
const CONVERTION_INT :f64 = 32f64;

fn main() {
    println!("Celsius <-> Fahrenheit converter");
    let is_celsius: bool = fahrenheit_or_celsius();
    let temp_to_convert: f64 = get_temperature(is_celsius);
    math_converter(temp_to_convert, is_celsius);
}

fn math_converter(source_temp: f64, f_o_c: bool) {
    let mut temperatures: [&str; 2] = ["Fahrenheit", "Celsius"];
    let final_temp: f64;
    if f_o_c {
        temperatures.reverse();
        final_temp = source_temp * CONVERTION_RATIO + CONVERTION_INT;
    }
    else {
        final_temp = (source_temp - CONVERTION_INT) / CONVERTION_RATIO;
    }
    println!("{} {} degrees are equivalent to {} {}", source_temp, temperatures[0], final_temp, temperatures[1]);
}

fn fahrenheit_or_celsius() -> bool {
    loop {
        println!("Do you wish to convert from (C)Celsius or from (F)Fahrenheit");

        let mut source_format: String = String::new();
        io::stdin().read_line(&mut source_format)
            .expect("Failed to read line");

        if source_format.trim().eq_ignore_ascii_case("c") {
            return true
        } else if source_format.trim().eq_ignore_ascii_case("f") {
            return false
        }
    }
}

fn get_temperature(f_o_c: bool) -> f64 {
    let mut temperatures: [&str; 2] = ["Fahrenheit", "Celsius"];
    if f_o_c {
        temperatures.reverse();
    }
    loop {
        println!("Input {} temperature you wish to convert in {}", temperatures[0], temperatures[1]);

        let mut temperature: String = String::new();
        io::stdin().read_line(&mut temperature)
            .expect("Failed to read line");

        let return_temperature: f64 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        return return_temperature
    }
}
