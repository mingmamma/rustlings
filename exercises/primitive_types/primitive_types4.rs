// primitive_types4.rs
//
// Get a slice out of Array a where the ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let arr_slice: &[i32] = &arr[1..4];
    // Can coerce an all of an array into a slice 
    let _arr_slice2 = &arr[..];
    assert_eq!([2, 3, 4], arr_slice);
    assert_eq!(&[2, 3, 4], arr_slice);

    let vec: Vec<i32> = vec![1, 2, 3];  
    
    let vec_slice: &[i32] = &vec[0..vec.len()];
    // Equivelent range specification for slice
    // omission can be made when the start the range is 0
    // and the end of the range is length of the container
    let _vec_slice2 = &vec[..];
    
    assert_eq!([1, 2, 3], vec_slice);
}
