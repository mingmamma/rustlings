// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);
        
        // Rust does NOT seem to entertain the thought of creating just an Option wrapped value
        // as a synonym of Some wrapped value as in Scala, the following does NOT work
        // let optional_target = Option(target);

        // Make this an if let statement whose value is "Some" type
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec!(None);

        // `for var in collection ...` is the for loop construct of Rust
        // https://doc.rust-lang.org/book/ch03-05-control-flow.html#looping-through-a-collection-with-for
        // in this case, the collection in question is the start..end syntax that creates a Range struct
        // https://doc.rust-lang.org/std/ops/struct.Range.html
        // the specific variant of the Range, denoted by `..`, is the half-open range that includes the `start`
        // and exclude the `end`
        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // make this a while let statement - remember that vector.pop also
        // adds another layer of Option<T>. You can stack `Option<T>`s into
        // while let and if let.
        while let Some(integer_option) = optional_integers.pop() {
            if let Some(integer) = integer_option {
                assert_eq!(integer, cursor);
                cursor -= 1;
            }
        }

        assert_eq!(cursor, 0);
    }
}
