// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

fn main() {
    // following the read/write/own permission semantics in illustrated in
    // https://rust-book.cs.brown.edu/ch04-02-references-and-borrowing.html#references-change-permissions-on-paths
    
    // x: R + W + O, althogh 
    let mut x: i32 = 100;

    // due to W permission of x
    x += 100;

    // due to R permission of x
    println!("{x}");
    
    // x: R, though dropping the permission of transferring ownership does not mean much in this case due to the type of x is i32 which implements Copy trait
    // y: R + O, noting the significance of ownership of y to a value of type &i32
    // the lifetime/scope?! of y, which is a mutable reference to a value of i32, starts
    let y: &mut i32 = &mut x;

    // y does not have write permission since it was declared an immutable variable in the first place
    // let mut x1: i32 = 101;
    // y = &mut x1;

    // due to R permission of y
    println!("{y}");

    // *y: W
    // due to W permission of *y
    *y += 100;
    // the lifetime/scope of y end

    
    let z = &mut x;
    *z += 1000;
    // the above program does constitute a violation of the borrowing rule where there can not be
    // more than one mutable references to the same owned value at any given point. However, the compilation
    // only fails with code below where the mutable references in doubt are used, i.e. just creating simultaneous
    // mutable references and using the dereference operator associated DOES NOT fail borrow check?!
    // println!("{} and {}", y, z);
    assert_eq!(x, 1300);
}
