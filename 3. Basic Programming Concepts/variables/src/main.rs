use std::io::{self, Read};

fn main() {
    println!("Hello, world!");

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    println!("Shadowing Concept");

    let y = 5;
    let y = y + 5;

    {
        let y = y * 2;
        println!("The value of y in inner scope is: {y}");
    }

    println!("The value of y is {y}");

    println!("Type infer");

    let guess: u32 = "32".parse().expect("Not a number!");

    println!("This is the guess which is converted : {guess}");

    let floating_x = 2.0;
    let floating_y: f32 = 3.0;

    println!("Floating numbers x : {floating_x} and y : {floating_y}");

    println!("Numeric Operations");
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    println!(
        "sum{sum}, difference : {difference} 
    product : {product}, quotient : {quotient},  truncated : {truncated},
    remainder : {remainder}"
    );

    println!("Boolean Types");
    let t = true;

    let f: bool = false; // with explicit type annotation

    println!("true : {t} false: {f}");

    println!("Compound Data Types");
    let tup = (500, 6.4, 1);

    println!("Deconstructing a tuple by doing pattern matching");

    let (x, y, z) = tup;

    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");

    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    println!("The value of five_hundred is: {five_hundred}");
    println!("The value of six_point_four is: {six_point_four}");
    println!("The value of one is: {one}");

    println!("Compound Type 2 : Arrays");

    let arr = [1, 2, 3, 4, 5];

    println!("Accessing Array elements");

    println!("Please enter an array index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = arr[index];

    println!("The value of the array at index {index} is : {element}");
}
