// errors4.rs
//
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a
// hint.

// Assertion macro usage involving custom type `PositiveNonzeroInteger` relies on PartialEq for comparison
// Also relies on Debug Debbug for formatting
// https://doc.rust-lang.org/core/macro.assert_eq.html#
#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {

        let result = match value {
            n if n == 0 => Err(CreationError::Zero),
            n if n < 0 => Err(CreationError::Negative),
            _ => Ok(PositiveNonzeroInteger(value as u64)),
        };
        result
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
