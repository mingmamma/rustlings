// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.

// Beyond simple fieldless enum, (tuple-like?!) enum variant and struct-like enum variant
// https://doc.rust-lang.org/reference/items/enumerations.html#enumerations
#[derive(Debug)]
enum Message {
    Move{x: u32, y: u32},
    Echo(String),
    ChangeColor(u32, u32, u32),
    Quit,
}

// Define a method associated with an enum item with an implementation item
// in a similar fashion to defining methods of a struct item 
impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
