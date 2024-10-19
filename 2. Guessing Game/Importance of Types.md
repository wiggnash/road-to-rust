- RUST has strong static type system and also have type inference 
	- **type inference** : which means that allows the compiler to automatically deduce the types of variables and expressions without requiring explicit type annotations from the programmer.

## Rust Number Types

- i32 -> 32 bit number => default value of the numbers
- u32 -> unsigned 32 bit number
- i64 -> 644 bit number

## Shadowing
- Rust allows use to shadow the previous value of a variable with a new one
- this concept helps us to reuse the variable name rather than focusing use to create a two unique variables
- **this concept is used to convert the value from one type to another type**

```rust
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```


## read_line( )

- read_line adds an new line character to the end of the string after we enter the value and press enter " \n "
- trim eliminates these things also
- trim( ) -> removes white space from starting and ending
