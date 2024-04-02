// modules2.rs
//
// You can bring module paths into scopes and provide new names for them with
// the 'use' and 'as' keywords. Fix these 'use' statements to make the code
// compile.
//
// Execute `rustlings hint modules2` or use the `hint` watch subcommand for a
// hint.

mod delicious_snacks {
    // the use ... as ... declaration rebinds the path `fruits::PEAR`` to a local name `fruit``, for use by other items exactly belonging to the 
    // `delicious_snacks` module
    // https://doc.rust-lang.org/reference/items/use-declarations.html
    // the addtional pub modifier makes pub use, s.t. the `fruit` item are not ONLY for the use by items of the `delicious_snacks module`, which
    // mean that `fruit` is private by default, but it can be accessed by an item of the parent module of `delicious_snacks`, due to the pub change
    // https://doc.rust-lang.org/reference/items/use-declarations.html#use-visibility
    pub use fruits::PEAR as fruit;
    pub use veggies::CUCUMBER as veggie;

    // the `self` path segment resolves the path to the current module
    // used as the first segment of a path, there SEEM TO BE NO USE CASE for it
    // https://doc.rust-lang.org/reference/paths.html#self
    // use self::fruits::PEAR as fruit;
    // use self::veggies::CUCUMBER as veggie;    
    
    // use declarations bring `fruit` and `veggie` into the scope (the parent module `delicious_snackes`) of the info function 
    // pub fn info() {
    //     println!(
    //         "favorite snacks: {} and {}",
    //         self::fruit,
    //         self::veggie
    //     );        
    // }

    mod fruits {
        // Does this 'static mean anything ?!
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    // The scope of the main function include all the sibling items, i.e. `delicious_snacks` module
    // However, to enable access to the items of the `delicious_snacks` module, the visibility modifier pub is needed
    // i.e. info function is modifiled to pub
    // delicious_snacks::info();
    
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}
