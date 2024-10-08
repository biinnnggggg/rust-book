fn main() {
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
