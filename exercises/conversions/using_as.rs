// using_as.rs
//
// Type casting in Rust is done via the usage of the `as` operator. Please note
// that the `as` operator is not only used when type casting. It also helps with
// renaming imports. The explicit type casting with the `as` operator as those 
// between primitive types: https://doc.rust-lang.org/stable/std/keyword.as.html
//
// The goal is to make sure that the division does not fail to compile and returns the proper type as desinated in the first place
//
// Execute `rustlings hint using_as` or use the `hint` watch subcommand for a
// hint.

fn average(values: &[f64]) -> f64 { /* &[f64] denotes the parameter type of shared reference to a slice of f64 elements */
    let total: f64 = values.iter().sum::<f64>();
    // explict desugaring of / operator
    let average:f64 = std::ops::Div::div(total, (values.len() as f64));
    average
    
    // equivalently and more commonly seen
    // total / (values.len() as f64)
}

fn main() {
    let values: [f64; 4] = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}
