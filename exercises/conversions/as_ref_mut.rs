// as_ref_mut.rs
//
// AsRef and AsMut allow for cheap reference-to-reference conversions. Read more
// about them at https://doc.rust-lang.org/std/convert/trait.AsRef.html and
// https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectively.
//
// Execute `rustlings hint as_ref_mut` or use the `hint` watch subcommand for a
// hint.

// Obtain the number of bytes (not characters) in the given argument.
// Note that AsRef trait is often used in Rust as a trait bound of generic function implementations
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    // as_bytes method of str to turn a &str into &[u8]/slice of bytes
    arg.as_ref().as_bytes().len()
}

fn char_counter_as_ref_str<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

fn char_counter_as_ref_string<T: AsRef<String>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

fn num_sq<T: AsMut<u32>>(arg: &mut T) {
    *arg.as_mut() = arg.as_mut().pow(2);
    // *arg.as_mut() = (*arg.as_mut()).pow(2);
}

struct Document {
    info: String,
    content: Vec<u8>,
}

impl<T: ?Sized> AsMut<T> for Document
where
    Vec<u8>: AsMut<T>,
{
    fn as_mut(&mut self) -> &mut T {
        self.content.as_mut()
    }
}

fn caesar<T: AsMut<[u8]>>(data: &mut T, key: u8) {
    for byte in data.as_mut() {
        *byte = byte.wrapping_add(key);
    }
}

fn caesar_u8_slice(slice_data: &mut[u8], key: u8) {
    for ref_mut_u8_item in slice_data {
        *ref_mut_u8_item = ref_mut_u8_item.wrapping_add(key);
    }
}

fn obfusgate_string_slice(slice_data: &mut[String], key: &str) {
    for ref_mut_string_item in slice_data {
        // efficient in-place update through mutable reference
        ref_mut_string_item.push_str(key);
        
        // less-efficient solution to obtain result by processing String cloned via the given mutable reference
        // *ref_mut_string_item = ref_mut_string_item.clone() + key;
    }
}

fn terminate_with_zero<T: AsMut<Vec<u8>>>(data: &mut T) {
    // a non-generic helper function, which contains most of the functionality to help to minimize monomorphization overhead
    fn append_zero(data: &mut Vec<u8>) {
        let len = data.len();
        if len == 0 || data[len-1] != 0 {
            data.push(0);
        }
    }
    append_zero(data.as_mut());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_count_with_different_argument_types() {
        let s1: &str = "Cafe au lait";
        let s2: String = String::from("Cafe au lait");

        assert_eq!(byte_counter(s1), byte_counter(s2.clone()));
        // assert_eq!(char_counter(s1), char_counter(s2.clone()));
    }

    #[test]
    fn char_count_with_different_arguement_types() {
        let s1: Box<String> = Box::new(String::from("Cafe au lait"));
        let s2: String = String::from("Cafe au lait");

        // The following line would not work due to the absence of impl AsRef<String> for String in reality in std crate: 
        // https://doc.rust-lang.org/std/string/struct.String.html#trait-implementations
        // the general gap of reflexive implementation of AsRef trait is noted and commented:
        // https://doc.rust-lang.org/std/convert/trait.AsRef.html#reflexivity
        // assert_eq!(char_counter_as_ref_string(s2.clone()), 12);

        assert_eq!(char_counter_as_ref_string(s1.clone()), 12);
        assert_eq!(char_counter_as_ref_str(s2.clone()), 12);
    }    

    #[test]
    fn mut_box() {
        let mut num: Box<u32> = Box::new(3);
        // the argument value supplied to num_sq() method satisfies the trait bound requirment of the method
        // s.t. the type Box<u32> implements the trait AsMut<u32>
        // Generic trait implementation: https://doc.rust-lang.org/stable/std/convert/trait.AsMut.html#impl-AsMut%3CT%3E-for-Box%3CT,+A%3E
        // Concreate trait implementation on Box: https://doc.rust-lang.org/stable/std/boxed/struct.Box.html#impl-AsMut%3CT%3E-for-Box%3CT,+A%3E
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }

    #[test]
    fn generic_as_mut_and_concrete_caesar_test() {
        let mut u8_slice: &mut[u8] = &mut [1, 2, 3];
        caesar_u8_slice(u8_slice, 2);
        assert_eq!(u8_slice, [3, 4, 5]);

        caesar(&mut u8_slice, 1);
        assert_eq!(u8_slice, [4, 5, 6]);

        let mut u8_vec: Vec<u8> = vec![1, 2, 3];
        // type Vec<u8> implements AsMut<[u8]> to qualify for arguement to caesar() function
        // https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#impl-AsMut%3C%5BT%5D%3E-for-Vec%3CT,+A%3E
        caesar(&mut u8_vec, 2);
        assert_eq!(u8_vec, [3, 4, 5]);

        // custom struct type Document implements both AsMut<[u8]> and AsMut<Vec<u8>> as evidenced by the generic AsMut<T> impl block
        // implementation, wherein the concrete type [u8] and Vec<u8> would satisfy the required trait bound of the generic type parameter
        let mut doc = Document {
            info: String::from("Example"),
            content: vec![7, 9, 8],
        };
        caesar(&mut doc, 1);
        assert_eq!(doc.content, [8, 10, 9]);
    }

    #[test]
    fn terminate_with_zero_test() {
        let mut u8_vec: Vec<u8> = vec![1, 2, 3];
        // type Vec<u8> implements AsMut<Vec<u8>>
        // https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#impl-AsMut%3CVec%3CT,+A%3E%3E-for-Vec%3CT,+A%3E
        terminate_with_zero(&mut u8_vec);
        assert_eq!(u8_vec, [1, 2, 3, 0]);

        let mut doc = Document {
            info: String::from("Example"),
            content: vec![7, 9, 8],
        };
        terminate_with_zero(&mut doc);
        assert_eq!(doc.content, [7, 9, 8, 0]);
    }
}
