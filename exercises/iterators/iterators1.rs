// iterators1.rs
//
// When performing operations on elements within a collection, iterators are
// essential. This module helps you get familiar with the structure of using an
// iterator and how to go through elements within an iterable collection.
//
// Make me compile by filling in the `???`s
//
// Execute `rustlings hint iterators1` or use the `hint` watch subcommand for a
// hint.

// #[derive(Debug, PartialEq, Copy, Clone)]
// Derive Debug trait for a ShirtColor enum value to be printed in debug format
// Derive Copy trait for the value of ShirtColor enum to be passed in and consumed by a function while
// preserving the ownership of that value for use later
// Derive Copy trait requires the type to implement the Clone trait?!
#[derive(Debug, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    // Desugaring the &self parameter annotation in method to its original form
    // to show the capure effect of the self variable by the closure
    // fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
    fn giveaway(self: &Self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // the closure arguement of the unwrap_or_else call captures the self variable which is
        // a shared reference to an Inventory value and subsquently call the most_stocked method
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    fn test_iter_next() {
        let fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

        // three general variants of creating an iterator (a value that implements the std::iter::Iterator trait)
        // iter() creates an iterator with associated type given as type Item = & T, implying the resulting iterator
        // is one that iterate over the shared reference of the elements from the given collection
        // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.iter
        // https://doc.rust-lang.org/std/slice/struct.Iter.html#impl-Iterator-for-Iter%3C'a,+T%3E
        let mut iterable_of_fav_fruits = fav_fruits.iter();
    
        println!("Can still use variable owning the original collection from which an iter() method
        is called to create an iterator {:?}", fav_fruits);
    
        assert_eq!(iterable_of_fav_fruits.next(), Some(&"banana"));
        assert_eq!(iterable_of_fav_fruits.next(), Some(&"custard apple"));
        assert_eq!(iterable_of_fav_fruits.next(), Some(&"avocado"));
        assert_eq!(iterable_of_fav_fruits.next(), Some(&"peach"));
        assert_eq!(iterable_of_fav_fruits.next(), Some(&"raspberry"));
        assert_eq!(iterable_of_fav_fruits.next(), None);
    }

    test_iter_next();

    fn test_iter_adaptors() {
        let vec = vec![1, 2, 3];
        let iter_of_vec = vec.iter();

        // the closure argument passed to the iterator adapter method, i.e. map in this case,
        // is called lazily, i.e. they would be called ONLY WHEN the adapted iterator is consumed
        let mut map_closure_argument_call_count = 0;
        // Definining the arguement to the closure argument of the map method does NOT affect
        // the Item associated type of the resulting Iterator type?!
        let mapped_iter_1 = iter_of_vec.clone().map(|&x| {
            map_closure_argument_call_count += 1;
            x+1
        });
        println!("the closure arguement passed to map was called {} times", map_closure_argument_call_count);
        
        let mut map_closure_argument_call_count_2 = 0;
        let consumed_mapped_iter_2: Vec<_> = iter_of_vec.map(|x: &i32| {
            map_closure_argument_call_count_2 += 1;
            x+1            
        }).collect();
        println!("the closure arguement passed to map was called {} times", map_closure_argument_call_count_2);
    }

    test_iter_adaptors();

    fn test_closure_1() {
        let store = Inventory {
            shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
        };
    
        let user_pref1 = Some(ShirtColor::Red);
        let giveaway1 = store.giveaway(user_pref1);
        println!(
            "The user with preference {:?} gets {:?}",
            user_pref1, giveaway1
        );
    
        let user_pref2 = None;
        let giveaway2 = store.giveaway(user_pref2);
        println!(
            "The user with preference {:?} gets {:?}",
            user_pref2, giveaway2
        );
    }

    test_closure_1();

}
