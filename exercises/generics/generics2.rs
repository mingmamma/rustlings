// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.

use std::fmt::Debug;

// struct and its impl taking generic parameter that is unconstrainted
struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

// struct and its impl taking generic parameter with trait bound
// https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods
struct PairWrapper<T> {
    x_val: T,
    y_val: T,
}

impl<T: Debug + PartialOrd> PairWrapper<T> {
    fn compare_val_and_show(&self) -> () {
        if self.x_val >= self.y_val {
            println!("The larger member of the instance is {:?}", self.x_val);
        } else {
            println!("The larger member of the instance is {:?}", self.y_val);            
        }
    }
}

// blanket implementation, i.e. impl for any type, by taking generic parameter, given such type satisfying given trait bound
// noting the specifc use of where clause to constraint trait bound on the generic type's derivative type,  a feature not expressable
// by other trait bound specification syntax since they can only specifc the trait bound on the generic type itself
trait ShowWrappedInOption { /* a trait providing the impletenting types a method to debug print their value wrapped in option */
    // not terribly good contract as is in this signature to move the ownsership of the value into the method
    // given its functionality only require read access
    fn show_wrapped_in_option(self) -> ();
}

// https://doc.rust-lang.org/stable/rust-by-example/generics/where.html
// where clause is the only available syntax to specify the trait bound of T based on constraint on its derivative type Option<T>
// in reality, this is a contrived example s.t. the trait bound `T where Option<T>: Debug` can be reduced to `T where T: Debug` by
// the fact the Option<T> conditionally implements Debug trait if T implements Debug trait:
// https://doc.rust-lang.org/std/option/enum.Option.html#impl-Debug-for-Option%3CT%3E
impl<T> ShowWrappedInOption for T where Option<T>: Debug {
    // Noting the method/functions in impl block DO NOT take any generic type parameter in all these examples
    // the generic type parameter is taken in the impl signature ONLY
    fn show_wrapped_in_option(self) -> () {
    
        println!("{:?}", Some(self)); /* what could happen if wrapping Some on some odd stuff, e.g. Some(None) ?! */
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
