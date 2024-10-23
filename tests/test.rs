use std::io::Cursor;

use cinputs::{cread_stream, prelude::*, read_stream};

#[test]
fn test_string_input() {
    assert_eq!(456, string_input::<u32>(&String::from("456")).unwrap());
    assert_eq!(-20, string_input::<i8>(&String::from("-20")).unwrap());
    assert!(string_input::<u8>(&String::from("257")).is_err());
    assert!(string_input::<u8>(&String::from("-45")).is_err());
}

#[test]
fn test_cstring_input() {
    let constraint = StringConstraint {
        exclude_char: vec!['y', 'i', 'e'],
        include_char: vec!['u', 'j', '2'],
        max_len: 10,
        min_len: 5,
    };
    assert!(cstring_input(&String::from("uj2gl"), &constraint).is_ok());
    assert!(cstring_input(&String::from("u2gljYY"), &constraint).is_ok());

    assert!(cstring_input(&String::from("uj2gil"), &constraint).is_err());
    assert!(cstring_input(&String::from("u5555555"), &constraint).is_err());
    assert!(cstring_input(&String::from("uj2"), &constraint).is_err());
    assert!(cstring_input(&String::from("uj2ooooooo0"), &constraint).is_err());
}

#[test]
fn test_read_stream() {
    let string = String::from("-78");
    assert!(read_stream::<_, u8>(Cursor::new(string.clone())).is_err());
    assert!(read_stream::<_, i8>(Cursor::new(string.clone())).is_ok());
}

#[test]
fn test_cread_stream() {
    let constraint = StringConstraint {
        exclude_char: vec!['y', 'i', 'e'],
        include_char: vec!['u', 'j', '2'],
        max_len: 10,
        min_len: 5,
    };
    assert!(
        cread_stream::<_, String, _>(Cursor::new(String::from("uj2gil")), &constraint).is_err()
    );
    assert!(
        cread_stream::<_, String, _>(Cursor::new(String::from("u2gljYY")), &constraint).is_ok()
    );
}

#[test]
fn test_number_constraint() {
    let number_constraint = NumberConstraint {
        min_value: -20.0,
        max_value: 500.0,
    };
    assert!(number_constraint.validate(&50).is_ok());
    assert!(number_constraint.validate(&500).is_ok());
    assert!(number_constraint.validate(&-20).is_ok());
    assert!(number_constraint.validate(&-20.4).is_err());
    assert!(number_constraint.validate(&500.2).is_err());
}

#[test]
fn test_string_constraint() {
    let string_constraint = StringConstraint {
        exclude_char: vec!['a', 'b', 'c', 'd'],
        include_char: vec!['e', 'f', 'g', 'h'],
        max_len: 10,
        min_len: 5,
    };
    assert!(string_constraint.validate(&"efghb".to_string()).is_err());
    assert!(string_constraint.validate(&"efgh".to_string()).is_err());
    assert!(string_constraint
        .validate(&"efgh8888888888".to_string())
        .is_err());
    assert!(string_constraint.validate(&"efgh6".to_string()).is_ok());
    assert!(string_constraint.validate(&"efgh777".to_string()).is_ok());
}
