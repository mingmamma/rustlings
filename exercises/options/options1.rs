// options1.rs
//
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a
// hint.

// This function returns how much icecream there is left in the fridge.
// If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
// all, so there'll be no more left :(
fn maybe_icecream(time_of_day: u8) -> Option<u8> {
    // We use the 24-hour system here, so 10PM is a value of 22 and 12AM is a
    // value of 0 The Option output can gracefully handle edge cases where time_of_day > 23
    // due to modelling with the Option enum type. The rust-by-example contains a comparable
    // example that of an implementation that models would model the return type simply by a 
    // unsigned int type, and handle edge cases with unreachable! macro and panic-based error handling
    match time_of_day {
        // use match guards to further filter match arms
        // https://doc.rust-lang.org/stable/rust-by-example/flow_control/match/guard.html#guards
        n if 0 <= n && n < 22 => Some(5),
        n if 22 <= 22 && n < 24 => Some(0),
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        let icecreams = maybe_icecream(12);
        assert_eq!(icecreams.unwrap(), 5);
    }
}
