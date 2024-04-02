// primitive_types2.rs
//
// Fill in the rest of the line that has code missing! No hints, there's no
// tricks, just get used to typing these :)

fn main() {

    let my_first_initial: char = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    // https://doc.rust-lang.org/std/primitive.char.html#method.is_alphabetic
    // https://doc.rust-lang.org/std/primitive.char.html#method.is_numeric
    let your_character = '🍑';
    if your_character.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
}
