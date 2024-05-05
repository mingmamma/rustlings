// from_into.rs
//
// The From trait is used for value-to-value conversions. If From is implemented
// correctly for a type, the Into trait should work conversely. You can read
// more about it at https://doc.rust-lang.org/std/convert/trait.From.html
//
// Execute `rustlings hint from_into` or use the `hint` watch subcommand for a
// hint.

// explicit import not needed likely due to the trait is in prelude
// use std::convert::From;

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// We implement the Default trait to specify a useful fallback default value for the implemented type
// https://doc.rust-lang.org/std/default/trait.Default.html
impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

impl From<&str> for Person {

    // implementation to parse a value of &str into a value of Person
    // the intended use case and hence the signature type of the from method dictates that
    // the FROM trait is meant to handle infalliable conversion s.t. the return type of the
    // from() method is a non-null/non-error value of the conversion target type
    fn from(s: &str) -> Person {
        // If the length of the provided string is 0, then return the default of Person.
        if s.len() == 0 { 
            return Person::default(); 
        }
        // split the given string on the commas present in it. Extract the first element from the split operation and use it as the name. 
        // If the name is empty, then return the default of Person.
        let parts: Vec<&str> = s.split(",").collect();
        if parts.len() < 2 || parts[0].is_empty() { 
            return Person::default();
        }
        let name = parts[0].to_string();
        // Extract the other element from the split operation and parse it into a `usize` as the age. If while parsing the age, something goes wrong, 
        // then return the default of Person. Otherwise, then return an instantiated Person object with the results        
        if let Ok(age) = parts[1].parse::<usize>() {
            return Person {name, age};
        } else {
            return Person::default();
        }
    }
}

fn main() {
    // Use the `from` function with parameter of &str type enabled by implementing the From trait on the Person type
    let p1 = Person::from("Mark,20");
    // Into trait is automatically available by the virtue of From trait having been implemented
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        // Test that the default person is 30 year old John
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }
    #[test]
    fn test_bad_convert() {
        // Test that John is returned when bad string is provided
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_good_convert() {
        // Test that "Mark,20" works
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }
    #[test]
    fn test_bad_age() {
        // Test that "Mark,twenty" will return the default person due to an error in parsing age
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "Mike");
        assert_eq!(p.age, 32);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,man");
        assert_eq!(p.name, "Mike");
        assert_eq!(p.age, 32);
    }
}
