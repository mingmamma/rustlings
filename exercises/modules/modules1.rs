// modules1.rs
//
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a
// hint.

// The sausage_factory module is defined in a in-line fashion (s.t. the source code is not in a separate file with path searched by the compiler) 
// where two items are defined: get_secret_recipe function and make_sausage function
// Since the module and the main function are of sibling relation in the root crate, the main function has access to the module (as container) but
// not its content (the items defined in the module)
// Use `pub` to make one function item visible, while the visibility of the other function item is private by default 
mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

// Appealing to the rules of crates and modules, it follows that
// Assuming the current file defines a binary crate, a crate s.t. the main function exists within
// The top level anonymous module of the crate, the root `crate`, contains two items: main function and sausage_factory module
fn main() {
    sausage_factory::make_sausage();
}
