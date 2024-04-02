// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.

// Complete the `capitalize_first` function.
// "hello" -> "Hello"
// "" -> ""
// chars() method of string slice returns a specialised Iterator type std::str::Chars
// whose associated type of the elements iterated over is of char type
// https://doc.rust-lang.org/std/primitive.str.html#method.chars
// https://doc.rust-lang.org/std/str/struct.Chars.html#impl-Iterator-for-Chars%3C'a%3E
pub fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    let mut first_char_uped: String = match chars.next() {
        None => String::new(),
        // Sigil @ to annotate type in pattern works?!
        Some(first @char ) => first.to_uppercase().to_string(),
    };
    
    // as.str() method of the Iterator type std::str:Chars can be understood as the inverse
    // ot chars s.t. it takes the chars() generated Chars Iterator back to the original string slice, 
    // less the elements due to the mutations carried out on the Chars that have consume them
    // s.t. as_str() returns the rest of the string slice unchanged except the first char given
    // that the next() method has been called exactly once on the Chars Iterator
    first_char_uped = first_char_uped + chars.as_str();
    first_char_uped
}

// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter()
         .map(|&s| capitalize_first(s))
         .collect()
}

// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    let capitalized_vector = capitalize_words_vector(words);
    capitalized_vector.concat()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_unwrap_or_else() {
        let s1 = "hello".to_string();
        // the next line would NOT actually call the closure arguement in unwrap_or_else but
        // the closure definition itself in effect moved the ownership of the captured String variable
        assert_eq!(Some("hello".to_string()).unwrap_or_else(|| s1), "hello");
        // s.t subsequent attempt to access the captured String variable fails compilation since the ownership
        // of that variable has been moved out, and that variable is not longer valid
        // let s2 = s1;
        // s.t another failure for the same reason
        // assert_eq!(None.unwrap_or_else(|| s1), "hello");]

        let vec1 = vec!["hello".to_string()];
        // unwrap_or_else accept a closure arguement of trait bound FnOnce() -> T. FnOnce() is the most general trait implemented
        // by all closures. The actual closure arguement passed can be ones belonging to a more specific subset of closures.
        // In the following example, the function value Vec:new can be treated as a closure with trait bound FnOnce()+FnMut()+Fn().
        // One subclass of such clousures are those DO NOT capture outer variable from their enclosing environment, which are the
        // equivalents to a regular function.
        assert_eq!(Some(vec1).unwrap_or_else(Vec::new), 
        vec!["hello".to_string()]/* Can NOT use vec1 again for comparison as vec1 variable has been moved by Some application*/);
        assert_eq!(None.unwrap_or_else(Vec::<String>::new), Vec::<String>::new());
    }

    #[test]
    fn test_sort_by_key() {
        let mut vec_1 = [1, -3, 2];
        let mut clousure_call_count = 0;

        vec_1.sort_by_key(|int_elem: &i32| -> i32 {
            clousure_call_count = clousure_call_count + 1;
            int_elem.abs()
        });

        assert_eq!(vec_1, [1, 2, -3]);
        assert_eq!(clousure_call_count, 6);
    }

    // https://doc.rust-lang.org/stable/rust-by-example/fn/closures/input_functions.html
    #[test]
    fn test_supply_functions_as_closure_arguement() {
        fn apply_fn_closure_returning_i32<F: Fn()->i32>(f: F) -> i32  {
            f()
        }

        let var_fn_closure_returning_i32 = || 42;

        // Is it possible to extend the following treamentment of a function WITHOUT arguements as a Fn closure
        // to the treatment of a function WITH arguements as a FN closure?
        fn func_returning_i32() -> i32 {
            42
        }

        assert_eq!(apply_fn_closure_returning_i32(var_fn_closure_returning_i32), 42);
        assert_eq!(apply_fn_closure_returning_i32(func_returning_i32), 42);
    }

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }

    fn test_closure() {
        let outer_int_var = 42;

        // Clousure are able to refer to existing variables in its enclosing environment, which is not accessibble for functions
        // Parameter and return type annotation can be totally inferred for closure whereas they are mandatory for
        // a function. Optional explicit annotation can also be provided
        let inferred_int_increment_closure = |i| i + outer_int_var;
        let annotated_int_increment_closure = |i: i32| -> i32 {i + outer_int_var};
        let inferred_int_identity_closure = || outer_int_var; 

        assert_eq!(inferred_int_increment_closure(1), 43);
        assert_eq!(annotated_int_increment_closure(2), 44);
        assert_eq!(inferred_int_identity_closure(), 42);
    }
}
