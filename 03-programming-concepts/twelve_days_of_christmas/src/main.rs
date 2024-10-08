fn main() {
    let day = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ];

    let line = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];
    
    println!("The Twelve Days of Christmas\n");

    for i in 0..12 {
        println!("On the {} day of Christmas my true love sent to me", day[i]);
        for j in (1..i).rev() {
            println!("{},", line[j]);
        }
        println!("{}.\n", line[0]);
    }
}
