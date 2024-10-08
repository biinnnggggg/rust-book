use std::io;

fn main() {
    //no_type_annotations();
    //floating_point_types();
    //numeric_operations();
    //tuples();
    arrays();
}

fn no_type_annotations() {
    //let guess = "42".parse().expect("Not a number!");
}

fn floating_point_types() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}

fn numeric_operations() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let diff = 95.5 - 4.3;

    // multiplication
    let prod = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;

    // remainder
    let remainder = 43 % 5;
    
    println!("sum: {sum}");
    println!("difference: {diff}");
    println!("product: {prod}");
    println!("quotient: {quotient}");
    println!("truncated: {truncated}");
    println!("remainder: {remainder}");
}

fn tuples() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // destructure tuple via pattern matching
    let (x, y, z) = tup; 

    println!("The value of y is: {y}");

    // accessing tuple values via index
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("The 0-index value is: {five_hundred}");
}

fn arrays() {
    let array = [1, 2, 3, 4, 5];
    println!("array is: {:?}", array);

    // type annotations
    let array: [i32; 5] = [1, 2, 3, 4, 5]; // [type, length]

    // initializing to same value
    let new_array = [3; 5]; // [value, length]
    
    println!("new_array is: {:?}", new_array);

    // accessing array elements via indexing
    let first = array[0];
    let second = array[1];
    
    println!("The value of first is: {first}");
    println!("The value of second is: {second}");

    // invalide array element access
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = array[index];

    println!("The value of the element at index {index} is: {element}");
}