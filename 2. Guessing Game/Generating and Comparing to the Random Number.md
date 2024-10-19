![[Pasted image 20241019175902.png]]

```rust
use rand::Rng
```

rand -> crate or package
Rng -> Trait

---

## Comparing the guess to the secret number

```rust
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
```

**Ordering ( Enum )** type 
which has three variants => which means these three are the possible outcome when we compare two values
1. Less
2. Greater
3. Equal

**cmp**

```rust
guess.cmp(&secret_number)
```

1. guess -> The number that i types
2. &secret_number -> reference of the number that we want to compare 
3. Therefore we are comparing guess -> secret number
4. this cmp returns a variant of ordering

**match**
- this is used to decide what to do next , with the return value of cmp
- cmp returns an variant from the odering 
- odering can take three values , less , greater and equal

- match is made using arms
- an arm consist of patterns , which we have to matched and the code which has to execute if its matched

**patterns and match** : these are very powerful feature of rust 
*why is it powerful ?*

- it helps us to handle all the situations that our code might encounter