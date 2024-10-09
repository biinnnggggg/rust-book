fn main() {
    double_free_error();
    cloning();
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
