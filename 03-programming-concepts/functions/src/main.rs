fn main() {
    another_function(5);
    print_labeled_measurement(5, 'h');
    expressions();
    test_return();
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn expressions() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn five() -> i32 {
    5
}

fn test_return() {
    let x = five();
    println!("The value of x is: {x}");
}