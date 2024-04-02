// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

use std::iter;

fn trim_me(input: &str) -> String {
    // Remove whitespace from both ends of a string
    // https://doc.rust-lang.org/std/primitive.str.html#method.trim
    
    // input.trim().to_string()
    String::from(input.trim())
}

fn compose_me(input: &str) -> String {    
    // String::from(input) + " world!"
    // input.to_string() + " world!"
    
    // format macro has particular advantage over + operator for concatenating multiple strings
    // since it will be hard to reason the ownership relations where it invovles multiple + operators
    // https://doc.rust-lang.org/book/ch08-02-strings.html?highlight=string#concatenation-with-the--operator-or-the-format-macro
    // format!("{} world!", input)
    
    let mut s = String::from(input);
    s.push_str(" world!");
    s
}

fn replace_me(input: &str) -> String {
    // Replace "cars" in the string with "balloons"!
    // https://doc.rust-lang.org/std/primitive.str.html#method.replace
    input.to_string().replace("cars", "balloons")
}

fn first_word(s: &str) -> &str {
    // showing that a ASCII byte literal is a value of u8 type
    let byte_literal_demo_value:u8 = b' ';
    
    // as_bytes for str: string slice to bytes slice i.e. self: &str -> &[u8] 
    let s_bytes = s.as_bytes();

    // into_iter() enabled by implementation IntoIterator trait, and the iter() method, for slice type [T], or shared slice type &[T]
    // both have the same outcome: 
    // https://doc.rust-lang.org/std/primitive.slice.html#impl-IntoIterator-for-%26%5BT%5D
    // https://doc.rust-lang.org/std/primitive.slice.html#method.iter
    // Most notably, the elemented being iterated over is shared reference type:
    // https://doc.rust-lang.org/std/slice/struct.Iter.html#impl-Iterator-for-Iter%3C'a,+T%3E
    
    // for (index, &byte_ref) in s_bytes.into_iter().enumerate() {
    for (index, &byte_ref) in s_bytes.iter().enumerate() {

        if byte_ref == b' ' {
            // return &s[..index];
            return &s[0..index]
        }
    }

    // &s[..]
    &s[0..s.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        // a value of String type and a value of &'a str type can be compared for equality (with == operator)
        // and inequality (with != operator), by implementing this PartialEq trait variant
        // https://doc.rust-lang.org/std/primitive.str.html#impl-PartialEq%3Cstr%3E-for-String
        // assert_eq is then enabled by using the equality comparsion provided by the PartialEq trait
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
        assert_eq!(trim_me("\n Hello\tworld\t\n"), "Hello\tworld")
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }

    #[test]
    fn find_first_substring_before_space() {
        assert_eq!(first_word("HelloWorld"), "HelloWorld");
        assert_eq!(first_word("Hello World"), "Hello");
    }

    // example of bad lifetime of referents that fails compilation
    // #[test]
    // fn find_first_substring_before_space_bad_lifetime() {
    //     let r;
    //     {
    //         let s = String::from("Hello World");
    //         r = first_word(&s);
    //     }
    //     assert_eq!(r, "Hello");
    // }
}
