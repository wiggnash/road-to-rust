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
}
