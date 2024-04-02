// traits1.rs
//
// Time to implement some traits! Your task is to implement the trait
// `AppendBar` for the type `String`. The trait AppendBar has only one function,
// which appends "Bar" to any object implementing this trait.
//
// Execute `rustlings hint traits1` or use the `hint` watch subcommand for a
// hint.

trait AppendBar {
    fn append_bar(self) -> Self;
    fn append_bar_1(self) -> Self;
    fn append_bar_2(self) -> Self;        
}

impl AppendBar for String {
    // Implementations of `AppendBar` for type `String`.
    fn append_bar(mut self) -> String {
        self + "Bar"
    }

    fn append_bar_1(mut self) -> String {
        // format!("{}Bar", self)
        format!("{self}Bar")
    }

    fn append_bar_2(mut self) -> String {
        self.push_str("Bar");
        self
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar_1().append_bar_2(),
            String::from("BarBar")
        );
    }
}
