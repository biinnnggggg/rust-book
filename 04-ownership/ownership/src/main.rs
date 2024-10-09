fn main() {
    double_free_error();
    cloning();
    ownership();
}

fn double_free_error() {
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{s1}, world!"); 
}

fn cloning() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
}

fn ownership() {
    let s = String::from("hello");

    takes_ownership(s);

    let x = 5;

    makes_copy(x);

    //println!("{x}");
    //println!("{s}");
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}