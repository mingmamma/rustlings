// enums1.rs
//
// No hints this time! ;)

// field-less enum with all unit enum variants
// https://doc.rust-lang.org/reference/items/enumerations.html#unit-only-enum
#[derive(Debug)]
enum Message {
    Quit,
    Echo,
    Move,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
