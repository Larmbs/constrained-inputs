use std::usize;

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
