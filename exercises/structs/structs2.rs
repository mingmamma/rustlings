// structs2.rs
//
// Execute `rustlings hint structs2` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug)]
struct Order {
    name: String,
    year: u32,
    made_by_phone: bool,
    made_by_mobile: bool,
    made_by_email: bool,
    item_number: u32,
    count: u32,
    foo: String,
}

fn create_order_template() -> Order {
    Order {
        name: String::from("Bob"),
        year: 2019,
        made_by_phone: false,
        made_by_mobile: false,
        made_by_email: true,
        item_number: 123,
        count: 0,
        foo: String::from("bar"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn your_order() {
        let order_template = create_order_template();
        let your_order = Order {
            name: String::from("Hacker in Rust"),
            count: 1,
            // functional update syntax using struct base
            // https://doc.rust-lang.org/reference/expressions/struct-expr.html
            // https://doc.rust-lang.org/reference/expressions/struct-expr.html#functional-update-syntax
            ..order_template
        };

        // the foo field of order_template struct would be unavailable due to it being a String type s.t.
        // when used with struct update syntax, the ownership of the String value of the field is moved 
        // into the new variable your_order. Replicating the issue in the last part the following account:
        // https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-from-other-instances-with-struct-update-syntax
        // assert_eq!(order_template.foo, "Bar");
        
        assert_eq!(your_order.name, "Hacker in Rust");
        assert_eq!(your_order.year, order_template.year);
        assert_eq!(your_order.made_by_phone, order_template.made_by_phone);
        assert_eq!(your_order.made_by_mobile, order_template.made_by_mobile);
        assert_eq!(your_order.made_by_email, order_template.made_by_email);
        assert_eq!(your_order.item_number, order_template.item_number);
        assert_eq!(your_order.count, 1);
    }
}
