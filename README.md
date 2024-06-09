# Constrained Inputs
Constrained Inputs is a Rust crate that simplifies the process of handling user input with specific type constraints and validations. This crate provides a mechanism to define and apply constraints to ensure that user inputs meet certain criteria before being accepted.

## Features
- Easy-to-use input parsing and prompting
- Customizable constraints for strings and numbers
- Detailed error handling for invalid inputs and constraint violations

# Usage
### Basic Input Parsing
You can use the input function to parse user input directly into a specified type.

``` rust
use constrained_inputs::input;

fn main() {
    let int: i32 = input().expect("Input was invalid");
    println!("Your input integer: {}", int);
}
```

### Constrained Input Parsing
Use the constrained_input function to prompt user input with additional constraints.

``` rust
use constrained_inputs::{constrained_input, constraints::NumberConstraint};

fn main() {
    let constraint = NumberConstraint {
        max: Some(100),
        min: Some(10),
    };
    let int: i32 = constrained_input(constraint).expect("Input was invalid or out of range");
    println!("Your constrained input integer: {}", int);
}
```

### Constraints
The constraints module provides various constraint configurations that can be applied to user inputs. These include constraints for strings and numbers.

#### String Constraints
Define constraints on strings, such as maximum length, minimum length, and blacklisted characters.

```rust
use constrained_inputs::constraints::{StringConstraint, Constraint, ConstraintResult, ConstraintError};

fn main() {
    let string_constraint = StringConstraint {
        max_len: Some(10),
        min_len: Some(5),
        blacklist_chars: vec!['a', 'e', 'i', 'o', 'u'],
    };

    let result = string_constraint.validate(&"hello");
    assert_eq!(result, ConstraintResult::Err(ConstraintError::BlacklistedChar));
}
```

#### Number Constraints
Define constraints on numbers, such as maximum and minimum values.

```rust
use constrained_inputs::constraints::{NumberConstraint, Constraint, ConstraintResult};

fn main() {
    let number_constraint = NumberConstraint {
        max: Some(100),
        min: Some(10),
    };

    let result = number_constraint.validate(&50);
    assert_eq!(result, ConstraintResult::Valid);
}
```

### Input Stream
A function that reads in a stream of data from a bufreader.

```rust
fn main() {
    let cursor = io::Cursor::new("123");

    let res = input_stream::<i32, _>(cursor);

    assert_eq!(123, res.unwrap());
}
```
