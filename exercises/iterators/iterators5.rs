// iterators5.rs
//
// Let's define a simple model to track Rustlings exercise progress. Progress
// will be modelled using a hash map. The name of the exercise is the key and
// the progress is the value. Two counting functions were created to count the
// number of exercises with a given progress. Recreate this counting
// functionality using iterators. Try not to use imperative loops (for, while).
// Only the two iterator methods (count_iterator and count_collection_iterator)
// need to be modified.
//
// Execute `rustlings hint iterators5` or use the `hint` watch subcommand for a
// hint.

use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Progress {
    None,
    Some,
    Complete,
}

// Given a Progress variant, return the count of entries in the HashMap of exerecise name (String) to Progress having the specific Progress variant
// Implementation with for construct, 
fn count_for(map: &HashMap<String, Progress>, value: Progress) -> usize {
    let mut count = 0;
    // values() method of HashMap returns an Iterator with item type &V, given HashMap<K, V>
    for &val in map.values() {
        if val == value {
            count += 1;
        }
    }
    count
}

fn count_iterator(map: &HashMap<String, Progress>, value: Progress) -> usize {
    // iter() method of HashMap returns an Iterator with item type of tuple of (&K, &V), given HashMap<K, V>
    map.iter()
       .filter(|&map_entry| *(map_entry.1) == value).count()
    // .filter(|entry| *((*entry).1) == value).count()   the sole arguement of filter is of type &Self::item
    // s.t that type would be &(&String, &Progress in this case) s.t without pattern matching of & would result in brainf*ck code 
}

fn count_collection_iterator(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    collection.iter()
              .map(|hash_map_instance| count_iterator(hash_map_instance, value)) /* compose by reusing implementation */
              .sum()
}

// lecture on implementation of flatten functionality for qualified iterators: https://www.youtube.com/watch?v=yozQ9C69pNs

// public interface that takes the input and return the flattened iterator transformed from the input, 
// an API that follows the pattern of the class of APIs on iterator of so-called the iterator adapters
fn flatten_fn<I>(to_be_flattened: I) -> Flatten<I::IntoIter> 
where 
    I: IntoIterator, <I as IntoIterator>::Item: IntoIterator 
{
    Flatten::new(to_be_flattened.into_iter())
}

// one approach to further enhance the ergonomics of public flatten_fn method that is a free-standing function
// is to provide the same API under the well-known Iterator trait
// Such change would in reality extend the the Iterator trait with custom functionaily from what's already given in the std crate
// in a similar fashion to extension methods work in Scala. 
// The following illustrates the idiomatic way to achieve such a task in Rust where is solution involves the creation an extension trait
// whose supertrait is the existing trait intended to be extended that include the extension method of our own. To make the extension 
// method available under the existing trait, the blanket implementation technique is used so that to types implementing the existing 
// trait, the extension trait would also be implemented, making the desired extension method visible as a member of the exsiting trait

trait IteratorExtension
{
    fn flattend(self: Self) -> Flatten<Self> 
    where 
        Self: Iterator + Sized, <Self as Iterator>::Item: IntoIterator;
}

impl<T> IteratorExtension for T 
{
    fn flattend(self: T) -> Flatten<T> 
        where 
            T: Iterator, <T as Iterator>::Item: IntoIterator {
        Flatten::new(self)
    }
}

struct Flatten<O> 
where 
    O: Iterator, O::Item: IntoIterator
{
    outer_iter: O,
    inner_next_iter: Option<<O::Item as IntoIterator>::IntoIter>,
    inner_back_iter: Option<<O::Item as IntoIterator>::IntoIter>
}

impl<O> Flatten<O>
where 
    O: Iterator, <O as Iterator>::Item: IntoIterator
{
    fn new(to_be_flattened_iter: O) -> Self {
        Self {
            outer_iter: to_be_flattened_iter,
            inner_next_iter: None,
            inner_back_iter: None,
        }
    }
}

impl<O> Iterator for Flatten<O> 
where 
    O: Iterator, 
    O::Item: IntoIterator
{
    type Item = <O::Item as IntoIterator>::Item;

    // the implementation of next would involve recursion to deal with cases where the `inner_next_iter` field is empty so that
    // the `outer_iter` field needs to be iterated to produce the next inner item, if any. Recursion is needed since the immediately
    // next inner item may well be empty which would be correctly handled with ignorance by the recursion logic
    // the recursion can be equivalently implemented in a loop form as opposed to making next call within itself, as exhibited below
    fn next(&mut self) -> Option<Self::Item> {
        // an incorrect implementation for iterating the flattened iterator naively iterates the outer iterator and then iterates
        // what is obtained from that, if anything, to yield the result as desired output
        // self.outer_iter.next()
        //     .and_then(|inner_next_item| inner_next_item.into_iter().next())

        // the equivalent flawed implementation in two lines
        // let next_element_of_outer_iter = self.outer_iter.next()?;
        // next_element_of_outer_iter.into_iter().next()        

        // The flaw of such implementations lies in the fact that each next call of the flattened iterator would iterate the outer
        // iterator field for a new inner item while discard the previously obtained inner item, disregarding the very possibility
        // that the obtained inner item could have yielded many items for the final desired result.
        // As such, rectifying the flaw involves keeping track of the inner item yielded by the outer iter to further check if more 
        // results might be coming out of it, and thus requires the `Flatten` struct to model the inner item with an Option field

        match self.inner_next_iter {
            None => {
                match self.outer_iter.next() {
                    None => {
                        self.inner_back_iter.as_mut()?.next()
                    },
                    Some(outer_iter_next_item) => {
                        self.inner_next_iter = Some(outer_iter_next_item.into_iter());
                        self.next()
                    }
                }

            },
            Some(ref mut ref_mut_inner_next_iter) => {
                match ref_mut_inner_next_iter.next() {
                    Some(item) => Some(item),
                    None => {
                        self.inner_next_iter = None;
                        self.next()
                    }
                }
            }
        }
    }
}

impl<O> DoubleEndedIterator for Flatten<O>
where 
    O: Iterator + DoubleEndedIterator,
    O::Item: IntoIterator,
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator
{    
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut ref_mut_inner_back_iter) = self.inner_back_iter {
                if let Some(item) = ref_mut_inner_back_iter.next_back() {
                    return Some(item);
                } else {
                    self.inner_back_iter = None;
                }
            } else {
                if let Some(outer_iter_next_back_item) = self.outer_iter.next_back() {
                    self.inner_back_iter = Some(outer_iter_next_back_item.into_iter());
                } else {
                    // return 
                        // (match self.inner_next_iter {
                        //     None => None,
                        //     Some(ref mut ref_mut_inner_next_iter) => ref_mut_inner_next_iter.next_back(),
                        // });

                    // concise and equivalent one-liner implementation, noting the equivalence in the two approaches to
                    // obtain a mutable reference to the value held in the self struct's Option field `inner_next_iter`
                    // without affecting the ownership of that value from the struct, either via `ref mut` or via `as_mut` call
                    return self.inner_next_iter.as_mut()?.next_back();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dummy_flatten() {
        // Vec[Empty(Vec[()])]
        let mut dummy: Flatten<std::vec::IntoIter<Vec<()>>> = flatten_fn(Vec::<Vec<()>>::new());
        assert_eq!(dummy.next(), None);
    }

    #[test]
    fn empty_flatten() {
        let mut empty: Flatten<std::iter::Empty<Vec<()>>> = flatten_fn(std::iter::empty::<Vec<()>>());
        assert_eq!(empty.next(), None);
    }

    #[test]
    fn outer_once_inner_empty_flatten() {
        // Vec[Vec[()]]
        let mut outer_once_inner_empty: Flatten<std::iter::Once<Vec<()>>> = flatten_fn(std::iter::once(Vec::<()>::new()));
        assert_eq!(outer_once_inner_empty.next(), None);
    }

    #[test]
    fn outer_wide_inner_empty_flatten() {
        // Vec[Vec[()], Vec[()]]
        let mut outer_wide_inner_empty_flatten = flatten_fn(vec![Vec::<()>::new(), Vec::<()>::new()]);
        assert_eq!(outer_wide_inner_empty_flatten.next(), None)
    }

    #[test]
    fn one_in_flatten() {
        // Vec[Vec[1]]
        let mut one_one = flatten_fn(vec![Vec::from([1])].into_iter());
        assert_eq!(one_one.next(), Some(1));
        assert_eq!(one_one.next(), None);
    }

    #[test]
    fn two_in_flatten() {
        // Vec[Vec[1, 2]]
        let mut one_wide = flatten_fn(std::iter::once(Vec::from([1, 2])));
        assert_eq!(one_wide.next(), Some(1));
        assert_eq!(one_wide.next(), Some(2));
        assert_eq!(one_wide.next(), None);    
    }

    #[test]
    fn two_in_two_out_flatten_back_forth_evenly() {
        // Vec[Vec[1,2], Vec[3,4]]
        let mut two_in_two_out = flatten_fn(vec![vec![1,2], vec![3,4]]);
        assert_eq!(two_in_two_out.next(), Some(1));
        assert_eq!(two_in_two_out.next_back(), Some(4));
        assert_eq!(two_in_two_out.next_back(), Some(3));
        assert_eq!(two_in_two_out.next(), Some(2));
        assert_eq!(two_in_two_out.next(), None);
        assert_eq!(two_in_two_out.next_back(), None);
    }

    #[test]
    fn four_in_flatten_back_forth() {
        // Vec[Vec[1,2,3,4]]
        let mut four_in = flatten_fn(vec![vec![1,2,3,4]]);
        assert_eq!(four_in.next(), Some(1));
        assert_eq!(four_in.next_back(), Some(4));
        assert_eq!(four_in.next_back(), Some(3));
        assert_eq!(four_in.next(), Some(2));
        assert_eq!(four_in.next(), None);
        assert_eq!(four_in.next_back(), None);
    }

    #[test]
    fn two_in_two_out_flatten_forward_majority() {
        // Vec[Vec[1,2], Vec[3,4]]
        let mut two_in_two_out = flatten_fn(vec![vec![1,2], vec![3,4]]);
        assert_eq!(two_in_two_out.next(), Some(1));
        assert_eq!(two_in_two_out.next(), Some(2));
        assert_eq!(two_in_two_out.next(), Some(3));
        assert_eq!(two_in_two_out.next_back(), Some(4));
        assert_eq!(two_in_two_out.next_back(), None);
        assert_eq!(two_in_two_out.next(), None);        
    }

    #[test]
    fn two_in_two_out_flatten_back_majority() {
        // Vec[Vec[1,2], Vec[3,4]]
        let mut two_in_two_out = flatten_fn(vec![vec![1,2], vec![3,4]]);
        assert_eq!(two_in_two_out.next_back(), Some(4));
        assert_eq!(two_in_two_out.next_back(), Some(3));
        assert_eq!(two_in_two_out.next_back(), Some(2));
        assert_eq!(two_in_two_out.next(), Some(1));
        assert_eq!(two_in_two_out.next(), None);
        assert_eq!(two_in_two_out.next_back(), None);
    }    

    #[test]
    fn two_wide_flatten() {
        // Vec[Vec[1], Vec[2]]
        let mut two_wide = flatten_fn(vec![vec![1], vec![2]]);
        assert_eq!(two_wide.next(), Some(1));
        assert_eq!(two_wide.next(), Some(2));
        assert_eq!(two_wide.next(), None);
    }

    #[test] 
    fn three_some_flatten() {
        let three_some_data = vec![vec![2, 0, 4, 8], vec![], vec![6, 6, 6]];
        let mut three_some = flatten_fn(three_some_data);
        assert_eq!(three_some.next(), Some(2));
        assert_eq!(three_some.next(), Some(0));
        assert_eq!(three_some.next(), Some(4));
        assert_eq!(three_some.next(), Some(8));
        assert_eq!(three_some.next(), Some(6));
        assert_eq!(three_some.next(), Some(6));
        assert_eq!(three_some.next(), Some(6));
        assert_eq!(three_some.next(), None);
    }  

    // test for the API with improved ergonomics
    #[test] 
    fn three_some_flatten_2() {
        let three_some_data = vec![vec![2, 0, 4, 8], vec![], vec![6, 6, 6]];
        let mut three_some = three_some_data.into_iter().flattend();
        assert_eq!(three_some.next(), Some(2));
        assert_eq!(three_some.next(), Some(0));
        assert_eq!(three_some.next(), Some(4));
        assert_eq!(three_some.next(), Some(8));
        assert_eq!(three_some.next(), Some(6));
        assert_eq!(three_some.next(), Some(6));
        assert_eq!(three_some.next(), Some(6));
        assert_eq!(three_some.next(), None);
    }    

    #[test]
    fn count_complete() {
        let map = get_map();
        assert_eq!(3, count_iterator(&map, Progress::Complete));
    }

    #[test]
    fn count_some() {
        let map = get_map();
        assert_eq!(1, count_iterator(&map, Progress::Some));
    }

    #[test]
    fn count_none() {
        let map = get_map();
        assert_eq!(2, count_iterator(&map, Progress::None));
    }

    #[test]
    fn count_complete_equals_for() {
        let map = get_map();
        let progress_states = vec![Progress::Complete, Progress::Some, Progress::None];
        for progress_state in progress_states {
            assert_eq!(
                count_for(&map, progress_state),
                count_iterator(&map, progress_state)
            );
        }
    }

    #[test]
    fn count_collection_complete() {
        let collection = get_vec_map();
        assert_eq!(
            6,
            count_collection_iterator(&collection, Progress::Complete)
        );
    }

    #[test]
    fn count_collection_some() {
        let collection = get_vec_map();
        assert_eq!(1, count_collection_iterator(&collection, Progress::Some));
    }

    #[test]
    fn count_collection_none() {
        let collection = get_vec_map();
        assert_eq!(4, count_collection_iterator(&collection, Progress::None));
    }

    fn get_map() -> HashMap<String, Progress> {
        use Progress::*;

        let mut map = HashMap::new();
        map.insert(String::from("variables1"), Complete);
        map.insert(String::from("functions1"), Complete);
        map.insert(String::from("hashmap1"), Complete);
        map.insert(String::from("arc1"), Some);
        map.insert(String::from("as_ref_mut"), None);
        map.insert(String::from("from_str"), None);

        map
    }

    fn get_vec_map() -> Vec<HashMap<String, Progress>> {
        use Progress::*;

        let map = get_map();

        let mut other = HashMap::new();
        other.insert(String::from("variables2"), Complete);
        other.insert(String::from("functions2"), Complete);
        other.insert(String::from("if1"), Complete);
        other.insert(String::from("from_into"), None);
        other.insert(String::from("try_from_into"), None);

        vec![map, other]
    }
}
