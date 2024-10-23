# CInputs

CInputs is a Rust library designed for easy input parsing with built-in constraints. It provides functionality for reading user input and applying various constraints to ensure the validity of that input.

## Features

- **Simple Input Parsing**: Easily read user input as different types (e.g., integers, strings).
- **Constraint Application**: Apply constraints to inputs, such as minimum and maximum values for numbers, or required characters in strings.
- **Custom Error Handling**: Use detailed error messages to handle validation failures gracefully.

## Example Usage
Here is a quick example of using input() and cinput().
```rust
use cinputs::prelude::*;

fn main() {
    println!("What is your favorite integer?");
    let favorite_integer: isize = input().unwrap();
    println!("Great Choice! {}", favorite_integer);

    println!("What is your age?");
    let num_constraint = NumberConstraint {
        min_value: 0.0,
        max_value: 120.0,
    };
    match cinput::<u8, _>(num_constraint) {
        Ok(age) => println!("You really don't look {}.", age),
        Err(err) => match err.kind {
            error::ErrorKind::ValidationError => println!("Thats not possible."),
            _ => eprintln!("{}", err),
        },
    }

    println!("What is the best name that contains a J?");
    let string_constraint = StringConstraint {
        exclude_char: Vec::new(),
        include_char: vec!['J'],
        max_len: usize::MAX,
        min_len: usize::MIN,
    };
    match cinput::<String, _>(string_constraint) {
        Ok(name) => print!("Great choice of name, {} is very nice!", name),
        Err(err) => match err.kind {
            error::ErrorKind::ValidationError => {
                println!("How did you fail to think of a name containing J!?!?")
            }
            _ => eprintln!("{}", err),
        },
    }
}

```

## Other Features
- **Input Parsing Using a BufReader** Reading input through a more general buffer reader
- **Constraint Trait** Implement custom traits on your types
- **Input Parsing Using String** Directly parse strings and apply constraint to them
