use std::io;

fn main() {
    const DEG: char = '\u{00B0}';
    const ARROW: char = '\u{2192}'; 

    println!("Temperature Converter");
    println!("(1) Fahrenheit -> Celsius \n(2) Celsius -> Fahrenheit");
    println!("Select (1) or (2):");

    let mut selection = String::new();

    io::stdin()
        .read_line(&mut selection)
        .expect("Failed to read line");

    let selection: usize = selection
        .trim()
        .parse()
        .expect("'{selection}' is not a number");
    

    if selection != 1 && selection != 2 {
        println!("'{selection}' is not a valid mode.");
        println!("Exiting...");
        return;
    } else {
        let mut temp = String::new();

        println!("Enter a temperature:");

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        let temp: f64 = temp
            .trim()
            .parse()
            .expect("'{temperature}' is not a number");
        
        if selection == 1 {
            let converted_temp = fahrenheit_to_celsius(temp);
            println!("{temp:.3}{DEG}F {ARROW} {converted_temp:.3}{DEG}C");
        } else if selection == 2 {
            let converted_temp = celsius_to_fahrenheit(temp);
            println!("{temp:.3}{DEG}C {ARROW} {converted_temp:.3}{DEG}F");
        } else {
            println!("Something went wrong! Exiting...");
        }
    }
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}
