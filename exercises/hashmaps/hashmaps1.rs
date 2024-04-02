// hashmaps1.rs
//
// A basket of fruits in the form of a hash map needs to be defined. The key
// represents the name of the fruit and the value represents how many of that
// particular fruit is in the basket. You have to put at least three different
// types of fruits (e.g apple, banana, mango) in the basket and the total count
// of all the fruits should be at least five.
//
// Make me compile and pass the tests!
//
// Execute `rustlings hint hashmaps1` or use the `hint` watch subcommand for a
// hint.

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    let mut basket: HashMap<String, u32> = HashMap::new();

    // Two bananas are already given for you :)
    // Added more fruits to pass the tests :)
    basket.insert(String::from("banana"), 2);
    basket.insert(String::from("apple"), 2);
    basket.insert(String::from("mango"), 2);
    basket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }

    #[test]
    fn exactly_two_apples() {
        let basket: HashMap<String, u32> = fruit_basket();
        let apple: String = String::from("apple");
        // get method is designed s.t. the parameter of get can be any borrowed form type of the key type, but NOT the key type itself
        // what consititute a borrow form of a given type ?! there should be a list
        // use &String as the following works
        // https://doc.rust-lang.org/std/collections/hash_map/struct.HashMap.html#method.get
        let apple_num = basket.get(&apple);
        assert_eq!(apple_num, Some(&2));
        // Even the equivalent value of shared reference to the string slice type works
        let apple_str = "apple";
        let apple_num_2 = basket.get(apple_str);
        assert_eq!(apple_num_2, Some(&2))
    }

    #[test]
    fn default_zero_non_existing_fruit() {
        let basket = fruit_basket();
        let pear = String::from("pear");
        // the return type of the get method on HashMap<K, V> is Option<&V>
        let pear_num_opt: Option<&u32> = basket.get(&pear);
        // further transform the option value as following
        // Option<&u32> to Option<u32> by the copied method of Option
        // https://doc.rust-lang.org/std/option/enum.Option.html#method.copied
        let pear_num: u32 = pear_num_opt.copied().unwrap_or(0);
        assert_eq!(pear_num, 0)
    }
}
