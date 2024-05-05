// vecs1.rs
//
// Your task is to create a `Vec` which holds the exact same elements as in the
// array `a`.
//
// Make me compile and pass the test!
//
// Execute `rustlings hint vecs1` or use the `hint` watch subcommand for a hint.

fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    // Expression of the comma-seperated list form of uniform type to initialise an array value
    // https://doc.rust-lang.org/reference/expressions/array-expr.html
    let a: [i32; 4] = [10, 20, 30, 40];
    
    // Use the vec! macro in prelude to create a vector value of type Vec<T>
    // https://doc.rust-lang.org/std/macro.vec.html
    let v: Vec<i32> = vec![10, 20, 30, 40];

    (a, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (arr, vec) = array_and_vec();
        // Obtain a value of the slice type [i32] from the vector v
        // Assigning the value to a variable would NOT compile unless one of the two usable forms of
        // slice type is given: shared reference or mutable reference thereof
        // change to let slice_of_v: &[i32] = &v[..] for example
        // let slice_of_v = v[..];
        
        // PartialEq implemented for comparing a value of Vec<i32> type to a value of [i32; 4] array type is enabled:
        // https://doc.rust-lang.org/std/vec/struct.Vec.html#impl-PartialEq%3C%5BU;+N%5D%3E-for-Vec%3CT,+A%3E
        // Note the type operated on is Vec<i32>. The trait in dual is NOT implemented on [i32: 4] type
        // hence the other way would NOT compile: assert_eq!(a, v)
        assert_eq!(vec, arr);

        assert_eq!(&arr[..], vec);
    }
}

fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];
    // v[2] is sugar of *v.index(2) where the return type of the index() would be &i32. The desugaring target of index() method as in the
    // `Index`= trait is from the immutable context. The addtional dereference operator * returns i32 type (as would be intuitive)
    // https://doc.rust-lang.org/std/ops/trait.Index.html
    let third_by_index: i32 = v[2];

    // In the mutable context here, v[2] is sugar of *v.index_mut(2) as of the `IndexMut` trait
    // https://doc.rust-lang.org/std/ops/trait.IndexMut.html
    v[2] = 30;

    // The for loop syntax is a sugar of using IntoIterator trait
    // https://doc.rust-lang.org/std/iter/index.html#for-loops-and-intoiterator
    // By the contract of IntoIterater trait, the collection that implements IntoIterator, and therefore
    // used in the sugared form of for loop, gives its ownership to the into_iter() method
    // https://doc.rust-lang.org/std/iter/trait.IntoIterator.html
    // This is a usually surprising and undesired outcome
    // https://doc.rust-lang.org/std/iter/index.html#iterating-by-reference
    // for i in v {
    //     println!("{i}");
    // }
    // the following would NOT compile since the preceeding for loop is sugar of the IntoIterator implementation of Vec that consumes the vec container
    // s.t. the vec has been moved and no longer available
    // https://doc.rust-lang.org/std/vec/struct.Vec.html#impl-IntoIterator-for-Vec%3CT,+A%3E
    // let first_by_index: i32 = v[0];

    // A low-effort workaround is to use the for loop with a shared reference to the collection
    // s.t. the ownership of the collection is not moved, as following
    for i in &v {
        println!("{i}");
    }


    let number_list = vec![34, 50, 25, 100, 65];
    
    // thre seems to be a subtlty here that `largest` is a mutable variable of shared reference of i32 type
    // rather than a value of a mutable reference of i32 type
    let mut largest: &i32 = &number_list[0];
    
    // this implementation of Vec explains the type of `number` to be &i32 in a pattern of `for bar in &vec_foo` 
    // https://doc.rust-lang.org/std/vec/struct.Vec.html#impl-IntoIterator-for-%26Vec%3CT,+A%3E
    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }
    
    println!("The largest number is {}", largest);

    let test_vec = vec!['a', 'b', 'c'];

    // the iter() method of std::slice module applies on the Vec<char> via Deref coersion machinary,
    // as if applied on a value of &[char]. Hence its return value implements Iterator<Item = &char>,
    // from which it can be concluded the variable pattern following the `for` keyword would result
    // in a variable matching type &char
    // https://doc.rust-lang.org/std/slice/struct.Iter.html#
    for ele in test_vec.iter() {
        todo!()
    }

    // the same reasoning also applies to the call with further application of enumerate() method
    // that returns an Enumerate iterator to conclude the types of matched tuple pattern
    // https://doc.rust-lang.org/std/iter/struct.Enumerate.html#
    for (index, value) in test_vec.iter().enumerate() {
        todo!()
    }
}