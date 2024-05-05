// move_semantics4.rs
//
// Refactor this code so that instead of passing `vec0` into the `fill_vec`
// function, the Vector gets created in the function itself and passed back to
// the main function.
//
// Execute `rustlings hint move_semantics4` or use the `hint` watch subcommand
// for a hint.

use std::ops::AddAssign;

fn main() {
    let vec1 = fill_vec();

    assert_eq!(vec1, vec![22, 44, 66, 88]);

    let mut vec2: Vec<i32> = vec![1, 2, 3];
        
    let ref_mut_third_element: &mut i32 = &mut vec2[2];

    // println!("the third element at this point is {}", vec2[2]);
        
    *ref_mut_third_element += 1;

    // One works, one doesn't :)
    // third_element += 1;
    ref_mut_third_element.add_assign(1);

    // Be mindful that the following line would constitute a move of a value of type &mut i32
    // let ref_third_element: &mut i32 = ref_mut_third_element;

    // Be mindful that the following line would create an immutable reference of data of `*ref_mut_third_element` s.t. during the lifetime of
    // that immutable reference, `*ref_mut_third_element` would not have persimision to write its data
    // let ref_third_element: &i32 = ref_mut_third_element;
    // Would-be equivalent statement with explicit dereference/reference operations
    let ref_third_element: &i32 = &*ref_mut_third_element;

    // s.t. this line to write data of `*ref_mut_third_element` does NOT have the permission and hence fails compilation
    // *ref_mut_third_element += 1;

    assert_eq!(*ref_third_element, 5);
    assert_eq!(*ref_mut_third_element, 5);
    
    assert_eq!(vec2[2], 5);
    
    // the following are stilled deemed as use of mutable borrows that would fail the compilation?!
    // assert_eq!(*ref_third_element, 5);
    // assert_eq!(*ref_mut_third_element, 5);
}

// `fill_vec()` no longer takes `vec: Vec<i32>` as argument - don't change this!
fn fill_vec() -> Vec<i32> {
    // create and fill the Vec in here within the function and return back out
    let mut vec = vec![22, 44, 66];
    vec.push(88);

    vec
}
