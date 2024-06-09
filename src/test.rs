use super::*;

#[test]
fn input_test1() {
    let cursor = io::Cursor::new("123");

    let res = input_stream::<i32, _>(cursor);

    assert_eq!(123, res.unwrap());
}

#[test]
fn string_constraints_test1() {
    let string_constraint = StringConstraint {
        max_len: Some(20),
        min_len: Some(2),
        blacklist_chars: vec!['a', 'b'],
    };

    let input = "Hello my name is dog"; // Should fail, has the letter A

    assert_ne!(ConstraintResult::Valid, string_constraint.validate(&input));

    let input = String::from("This is another sentence that is too long and has blacklisted chars");

    assert_ne!(ConstraintResult::Valid, string_constraint.validate(&input));
}

#[test]
fn number_constraint() {
    let number_constraint = NumberConstraint {
        max: Some(20),
        min: Some(-10),
    };

    let input: u8 = 15;

    assert_eq!(ConstraintResult::Valid, number_constraint.validate(&input));

    let input: u16 = 50;

    assert_ne!(ConstraintResult::Valid, number_constraint.validate(&input));

    let input: i16 = 50;

    assert_ne!(ConstraintResult::Valid, number_constraint.validate(&input));

    let input: i32 = -1000;

    assert_ne!(ConstraintResult::Valid, number_constraint.validate(&input));
}
