// lifetimes1.rs
//
// The Rust compiler needs to know how to check whether supplied references are
// valid, so that it can let the programmer know if a reference is at risk of
// going out of scope before it is used. Remember, references are borrows and do
// not own their own data. What if their owner goes out of scope?
//
// Execute `rustlings hint lifetimes1` or use the `hint` watch subcommand for a
// hint.

use std::string;

fn dangle() {
    let r;
    {
        let x: i32 = 42;
        r = &x;
    }
    // compilation only fails when actually using the dangle reference
    // println!("r: {}", r);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_2<'a>(x: &str, y: &str) -> &'a str {
    // this obvious offend the borrow checker for returning a reference to owned value scoped within the current function
    // let result = String::from("really long string");
    // result.as_str()

    // obviously fine
    let result = "really long string";
    result
}

fn main() {
    {
        let string1 = String::from("abcd");
        {
            let string2 = "xyz";
            let result = longest(string1.as_str(), string2);
            println!("The longest string is {}", result);
        }
    }

    {
        let string1 = String::from("abcd");
        let result;
        {
            let string_slice_2 = "xyz";
            result = longest(string1.as_str(), string_slice_2);
        }
        // This does NOT offend the borrow checker?!
        println!("The longest string is {}", result);
    }

    {
        let string1 = String::from("long long long");
        let result;
        {
            let string2 = String::from("xyz");
            result = longest(string1.as_str(), string2.as_str());
            // this placement of println! would compile
            println!("The longest string is {}", result);

        }
        // this placement println offend the borrow checker given the lifetime annotation
        // println!("The longest string is {}", result);
    }

    // dangle()
}
