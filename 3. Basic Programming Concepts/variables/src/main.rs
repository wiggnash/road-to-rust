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


    println!("sum{sum}, difference : {difference} 
    product : {product}, quotient : {quotient},  truncated : {truncated},
    remainder : {remainder}");

    println!("Boolean Types");
    let t = true;

    let f: bool = false; // with explicit type annotation

    println!("true : {t} false: {f}");
}
