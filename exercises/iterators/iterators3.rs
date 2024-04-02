// iterators3.rs
//
// This is a bigger exercise than most of the others! You can do it! Here is
// your mission, should you choose to accept it:
// 1. Complete the divide function to get the first four tests to pass.
// 2. Get the remaining tests to pass by completing the result_with_list and
//    list_of_results functions.
//
// Execute `rustlings hint iterators3` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug, PartialEq, Eq)]
pub enum DivisionError {
    // tuple-like enum variant (by only supplying the required tuple parameter type, i.e. custom struct type NotDevisibleError)
    NotDivisible(NotDivisibleError),
    DivideByZero,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotDivisibleError {
    dividend: i32,
    divisor: i32,
}

// Calculate `a` divided by `b` if `a` is evenly divisible by `b`.
// Otherwise, return a suitable error.
pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
   if b == 0 {
    Err(DivisionError::DivideByZero)
   } else if a % b == 0 {
    Ok(a / b)
   } else {
    Err(DivisionError::NotDivisible(NotDivisibleError { dividend: a, divisor: b }))
   }
}

// See the last example of the collect() method of an Iterator for using collect() method
// on an Iterator of whose associated type Self::item is of type Result<T, E>. By the
// type argument to the collect() method, and therefore the affected type of the collection returned
// by the collect() method, usually come in one of two forms
// CollectionType<Result<T, E>>, or Result<CollectionType<T>, E>
// Specifically, the samantics of a value of Result<CollectionType<T>, E> is that it is an Ok variant of
// a collection of values of type T, if the iterated over elements from the iterator that the collect() method
// operates on had returned all Ok variants of values of Result<T, E>. Otherwise, it is an Err varaint of
// the first Err variant returned by the iterator. 
// This behavior is intuitive BUT the the cause is still unclear?!

// Complete the function and return a value of the correct type so the test
// passes.
// Desired output of Result<Vec<i32>, DivisionError> in dbg format: Ok([1, 11, 1426, 3])
fn result_with_list() -> Result<Vec<i32>, DivisionError> {
    let numbers = vec![27, 297, 38502, 81];
    // Use variable binding type annotation to guide the return collection type
    let division_results: Result<Vec<i32>, DivisionError> = numbers.into_iter()
                                                                   .map(|n| divide(n, 27))
                                                                   .collect();
    division_results
}

// Complete the function and return a value of the correct type so the test
// passes.
// Desired output of Vec<Result<i32, DivisionError>> in dbg format: [Ok(1), Ok(11), Ok(1426), Ok(3)]
fn list_of_results() -> Vec<Result<i32, DivisionError>> {
    let numbers = vec![27, 297, 38502, 81];
    // Use turbofish syntax to supply type argument to the generic collect method to guide the return collection type
    let division_results = numbers.into_iter()
                                                                   .map(|n| divide(n, 27))
                                                                   .collect::<Vec<Result<_, DivisionError>>>();
    division_results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(
            divide(81, 6),
            Err(DivisionError::NotDivisible(NotDivisibleError {
                dividend: 81,
                divisor: 6
            }))
        );
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(result_with_list(), Ok(vec![1, 11, 1426, 3]));
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(list_of_results(), vec![Ok(1), Ok(11), Ok(1426), Ok(3)]);
    }
}
