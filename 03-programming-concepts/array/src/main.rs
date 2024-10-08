use std::io;

fn main() {
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