// from_str.rs
//
// This is similar to from_into.rs, but this time we'll implement `FromStr` and
// return errors instead of falling back to a default value. Additionally, upon
// implementing FromStr, you can use the `parse` method on strings to generate
// an object of the implementor type. You can read more about it at
// https://doc.rust-lang.org/std/str/trait.FromStr.html
//
// Execute `rustlings hint from_str` or use the `hint` watch subcommand for a
// hint.

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

// customised error type accopanying the `FromStr` implementation.
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Empty input string
    Empty,
    // Incorrect number of fields
    BadLen,
    // Empty name field
    NoName,
    // Wrapped error from parse::<usize>()
    ParseInt(ParseIntError),
}

// As an aside: `Box<dyn Error>` implements `From<&'_ str>`. This means that if
// you want to return a string error message, you can do so via just using
// return `Err("my error message".into())`.

impl FromStr for Person {
    type Err = ParsePersonError;
    
    // the FromStr trait of str module differs from the From<&str> trait most significantly
    // in its error semantics, s.t. its intended use case and hence the method signature of
    // the from_str() method returns a Result wrapped coversion target value that explicitly
    // models the possibility of error during conversion
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        // If the length of the provided string is 0, an error should be returned
        if s.len() == 0 {
            return Err(Self::Err::Empty);
        }
        // Split the given string on the commas present in it. Only 2 elements should be returned from the split, otherwise return an error
        let parts = s.split(",").collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(Self::Err::BadLen);
        }
        // Extract the first element from the split operation and use it as the name. If while extracting the name something goes wrong, an error should be returned
        if parts[0].is_empty() {
            return Err(Self::Err::NoName);
        }
        let name = parts[0];

        // https://doc.rust-lang.org/stable/rust-by-example/conversion/string.html#parsing-a-string
        // Extract the other element from the split operation and parse it into a `usize` as the age. If while extracting 
        // the age something goes wrong, an error should be returned
        // match parts[1].parse::<usize>() {
        //     Ok(age) => Ok(Person {
        //                 name: name.to_string(),
        //                 age,
        //                 }),
        //     Err(err) => Err(Self::Err::ParseInt(err))
        // }

        // alternative implementation to match on an Result enum make uses of the inherent method map_or_else that takes 
        // two closures as parameters for further transforming both the Ok and Err variants
        parts[1].parse::<usize>()
                .map_or_else(|err: ParseIntError| -> Result<Person, Self::Err> {
                Result::Err(Self::Err::ParseInt(err))}, 
             |age: usize| -> Result<Person, Self::Err> {
                // If everything goes well, then return a Result of a Person object
                Result::Ok(Person {name: name.to_string(), age,})}
                )
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;


    // Noting that converting &str to other types is commonly useful. Thus implementing FromStr trait for some type is useful since
    // that would enable the more commonly used parse() method of str to be available to parse &str input to the value of the custom
    // type. In this example, implementation for our custom type is needed but use case of parsing &str to common primitive types are
    // provided in std crate and supported by the same machinery.
    // https://doc.rust-lang.org/std/primitive.str.html#method.parse
    // https://doc.rust-lang.org/stable/rust-by-example/conversion/string.html#parsing-a-string
    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(ParsePersonError::Empty));
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }

    // matches! macro come handy for type testing the specific kind of error?!
    #[test]
    fn missing_age() {
        assert!(matches!(
            "John,".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!(
            "John,twenty".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(ParsePersonError::NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(
            ",".parse::<Person>(),
            // the or pattern?!
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!(
            "John,32,man".parse::<Person>(),
            Err(ParsePersonError::BadLen)
        );
    }
}
