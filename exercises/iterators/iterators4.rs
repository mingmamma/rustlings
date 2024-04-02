// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.

// bring into scope solely for explicit type annotation
use std::ops::RangeInclusive;
// use core::ops::RangeInclusive; /* If both work, what's the difference?! */

pub fn factorial_imperative_loops(num: u64) -> u64 {
    let mut result = 1;
    // Cast the immutable u64 argument num to a mutable variable s.t. it can be mutably decremented in each loop
    let mut num = num;
    
    loop {
       if num > 1 /* saves one iterator than would-be equivalent: if num > 0 */{
        result = result * num;
        num = num - 1;
       } else {
        break;
       }
    }

    result
}

pub fn factorial_while_loops(num: u64) -> u64 {
    let mut result = 1;
    let mut num = num;

    while num > 1 {
        result *= num;
        num = num - 1;
        
        // this attempt shadowing DOES NOT work, most likely due to scoping reason in that
        // the shadow effect is only WITHIN the bracketed {} body of the while construct, 
        // s.t that intended decremented by 1 with shadown would NOT be evaluated by the condition
        // of the the while construct. Leaving it uncommented DOES NOT affect the program :)
        let num = num - 1;
    }

    result
}

pub fn factorial_non_recurse(num: u64) -> u64 {

    if num == 0 {
        1
    } else {
        // ..= syntax signifies the range to include both the start and end element
        let range: RangeInclusive<u64> = 1..=num;
        
        // RangeInclusive type implements the Iterator trait that provides the fold method
        // By the description of fold(), it states it combines the elements in left-associated fashion, it seems that
        // supposedly imply for a ordered collection col = [x1, x2, .., xn], col.fold(acc, f) entails
        // f(f(f(acc, x1), x2...), xn). Combining in the right-associated fashion would entail
        // f(f(...f(acc, xn), x2), x1)
        // https://doc.rust-lang.org/core/iter/trait.Iterator.html#method.fold
        range.fold(1, |acc, x| acc * x)
    }
}

// implementation of an iterator to return Fib numbers. The two members are needed to maintain the current state of the iterator
#[derive(Clone)]
struct FibIter {
    small: u32,
    large: u32,
}

impl Iterator for FibIter {
    
    // required associated type and its value
    type Item = u32;
    
    // Require method and its signature
    fn next(self: &mut Self) -> Option<Self::Item> {
        // updating the state change caused by advancing the iterator
        let temp = self.small;
        self.small = self.large;
        self.large = temp + self.small;

        // return the result after advancing the iterator
        Some(temp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial_imperative_loops(0));
        assert_eq!(1, factorial_while_loops(0));
        assert_eq!(1, factorial_non_recurse(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial_imperative_loops(1));
        assert_eq!(1, factorial_while_loops(1));
        assert_eq!(1, factorial_non_recurse(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial_imperative_loops(2));
        assert_eq!(2, factorial_while_loops(2));
        assert_eq!(2, factorial_non_recurse(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial_imperative_loops(4));
        assert_eq!(24, factorial_while_loops(4));
        assert_eq!(24, factorial_non_recurse(4));
    }

    #[test]
    fn test_fib_iterator() {
        let fib_iter_instance = FibIter {small: 1, large: 1};
        
        // the `take` method of Iterator trait moves the ownership of the the original iterator into the method to produce a new iterator
        // as an instance of "iterator adapter", whose other notable examples include `map` method
        // As such methods mandates to move the ownership of original iterator, trying to supply a shared reference to the
        // original iterator to avoid its ownership from being moved is NOT allowed
        // let mut first_fibs = (&fib_iter_instance).take(3);

        let mut first_fibs = fib_iter_instance.clone().take(3);

        assert_eq!(first_fibs.next(), Some(1));
        assert_eq!(first_fibs.next(), Some(1));
        assert_eq!(first_fibs.next(), Some(2));

        let mut second_part_fibs = fib_iter_instance.skip(3)
                                                                               .take(3);
        assert_eq!(second_part_fibs.next(), Some(3));
        assert_eq!(second_part_fibs.next(), Some(5));
        assert_eq!(second_part_fibs.next(), Some(8));
    }
}
