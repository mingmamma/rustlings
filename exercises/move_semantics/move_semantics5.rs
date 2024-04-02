// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

#[test]
fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;    
    let z = &mut x;
    *z += 1000;
    // the above program does constitute a violation of the borrowing rule where there can not be
    // more than one mutable references to the same owned value at any given point. However, the compilation
    // only fails with code below where the mutable references in doubt are used, i.e. just creating simultaneous
    // mutable references and using the dereference operator associated DOES NOT fail borrow check?!
    // println!("{} and {}", y, z);
    assert_eq!(x, 1200);
}
