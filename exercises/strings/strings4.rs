// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    
    // to_owned
    // https://doc.rust-lang.org/std/borrow/trait.ToOwned.html
    string("rust is fun!".to_owned());
    
    // into enabled by this implementation
    // https://doc.rust-lang.org/std/primitive.str.html#impl-From%3C%26str%3E-for-String
    // and the relation between std::convert:FROM and std::convert::into trait
    // https://doc.rust-lang.org/std/convert/trait.Into.html#generic-implementations
    string_slice("nice weather".into());
    
    
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
        
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("Happy Monday!".replace("Mon", "Tue"));
    
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
