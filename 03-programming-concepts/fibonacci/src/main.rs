use std::io;

fn main() {
    println!("Input a number n:");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: u32 = n
        .trim()
        .parse()
        .expect("'{n}' is not a number");

    let fib_n = fib(n);

    println!("The {n}th fibonacci number is {fib_n}");

}

fn fib(n: u32) -> u64 {
    let mut a = 0;
    let mut b = 1;
    for _i in 0..n {
        (a, b) = (b, a + b);
    }
    a
}
