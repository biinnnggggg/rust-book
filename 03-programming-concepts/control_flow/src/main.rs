fn main() {
    //multiple_branches();
    //if_expressions();
    //branches();
    //loop_labels();
    //while_loops();
    //for_loops();
    for_loop_with_range();
}

fn multiple_branches() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn if_expressions() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

fn branches() {
    let condition = true;
    // let number = if condition { 5 } else { "six" };

    // println!("The value of number is: {number}");
}

fn loop_labels() {
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_loops() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loops() {
    let a = [10, 20, 30, 40, 50];
    
    for element in a {
        println!("The value is: {element}");
    }
}

fn for_loop_with_range() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    
    println!("LIFTOFF!!!");
}