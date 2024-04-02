// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.
use std::any::type_name;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // interaction of pattern match and reference
    // https://doc.rust-lang.org/stable/rust-by-example/flow_control/match/destructuring/destructure_pointers.html
    fn type_of<T>(_: T) -> &'static str {
        // for diagnostic use only :)
        type_name::<T>()
    }

    let ref_val: &i32 = &42;

    match ref_val {
        &val => println!{"val has the type {}", type_of(val)}
    }

    match ref_val {
        refed_val => println!{"refed_val_itself has the type {}", type_of(refed_val)}
    }

    match *ref_val {
        derefed_val => println!{"refed_val_itself has the type {}", type_of(derefed_val)}
    }


    // Use of ref keyword to bind by reference
    // https://doc.rust-lang.org/std/keyword.ref.html
    let immutable_point: Option<Point> = Some(Point { x: 100, y: 200 });

    let mut mutable_point: Option<Point> = Some(Point {x: 100, y: 200});

    match immutable_point {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }

    match mutable_point {
        Some(ref mut p) => {
            p.x *= 10;
            p.y *= 10;
            println!("Scaled mutable point by 10 fold: {},{} ", p.x, p.y)
        },
        _ => panic!("no match!"),
    }
    
    println!("{:?} is still available since its ownership has NOT been moved", immutable_point);
}
