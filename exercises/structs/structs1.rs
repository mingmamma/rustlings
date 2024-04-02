// structs1.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs1` or use the `hint` watch subcommand for a
// hint.

struct ColorClassicStruct {
    red: i32,
    green: i32,
    blue: i32,
}

struct ColorClassicStruct2 {
    red: i32,
    green: i32,
    blue: i32,
    label: String,
}

// Contrast the visibility modification on the same struct defined in a module
// To construct the comparable struct in a test function in the tests module,
// the `pub` keyword before the struct name enables accessing the struct under the `Foo` module
// whereas the `foo` module does NOT need `pub` keyword since it is already a visiable item in the
// parent module to the item of the test function in the child module `tests`
mod foo {
    pub struct ColorClassicStruct2 {
        red: i32,
        green: i32,
        blue: i32,
        // Selectively modify some fields to pub s.t. ONLY those fields are accessible and can be changed
        // by code in other modules
        // https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#making-structs-and-enums-public
        pub labels: Vec<String>, 
    }

    impl ColorClassicStruct2 {
        pub fn get_green() -> ColorClassicStruct2 {
            ColorClassicStruct2 {
                red: 0,
                green: 255,
                blue: 0,
                labels: vec!["green".to_string()], 
            }
        }
    }
}

struct ColorTupleStruct(i32, i32, i32);

#[derive(Debug)]
struct UnitLikeStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        let green = ColorClassicStruct{
            red: 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn classic_c_structs_2() {
        let mut green = foo::ColorClassicStruct2::get_green();

        green.labels = vec!["green".to_string(), "rainbow".to_string()]; // might be more proper to do this line with map

        let another_green = ColorClassicStruct2 {
            red: 0,
            green: 255,
            blue: 0,
            label: green.labels[0].clone(),
            // label: green.labels[0], ?!
        };
    }

    #[test]
    fn tuple_structs() {
        let green = ColorTupleStruct(
            0,
            255,
            0,
        );

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // Instantiate a unit-like struct in equivalent ways
        // https://doc.rust-lang.org/reference/expressions/struct-expr.html#unit-struct-expression
        let unit_like_struct = UnitLikeStruct;
        let unit_like_struct2: UnitLikeStruct = UnitLikeStruct{};
        let message = format!("{:?}s are fun!", unit_like_struct2);

        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}
