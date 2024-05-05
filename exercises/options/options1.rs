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

// an example involving move semantics concerning Options
// https://doc.rust-lang.org/stable/rust-by-example/error/option_unwrap/question_mark.html?highlight=option#unpacking-options-with-
#[derive(Clone)]
struct Person {
    job: Option<Job>,
}

// #[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

// implementing Copy trait of the PhoneNumber type is necessary for satisfying the specific rule s.t
// a struct implements the Copy trait 
// #[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {
    // method to get the area code of the phone number of the person's job, if it exists.
    fn work_phone_area_code(self: &Person) -> Option<u8> {
        
        // the move semantics enforced in the cases of the ? operator on Options is made clear with the following example
        // in the sense that the value in the Some variant of the `job` field of the `Person` struct, extracted by the the
        // ? operator and then assigned to a variable, follows the move semantics of copy or move of variable assignment
        // depending on whether the type of the value implements the Copy trait. 
        // In this specific implementation, the partial move of the field `job` is disallowed since the field is accessed
        // not from a shared reference to the `Person` struct, which should not mutate the `Person` struct caused by using
        // the shared reference
        
        // hence either implementing the Copy trait for the `job` type, or avoid using a owned value of Person type instead of
        // a shared reference would be valid workarounds
        // hence the following implemetation compiles only if Copy traits are implemented for Job and PhoneNumber types since
        // a struct type implements Copy trait ONLY IF all types of its fields implements the Copy trait?!
        // let persons_job: Job = self.job?;
        // let person_work_phone: PhoneNumber = persons_job.phone_number?;
        // person_work_phone.area_code

        // equivalent one-liner
        // self.job?.phone_number?.area_code

        // the idea of creating a owned value out of `self` which is a shared reference to a `Person` instance would be
        // implemented most intuitively via cloning self to return a owned `Person` instance s.t. it would depend on 
        // the Person type implements the Clone trait s.t. clone method on a value of `&Person` would return a value
        // of `Person` and NOT trivially a value of `&Person`. Doing that would be more involved than the alternative
        // solution given the rule that implementing Clone trait on a struct type would have required implementing 
        // Clone trait on the all types of its fields, comparable rule to Copy?!
        let owned_person: Person = self.clone();
        owned_person.job?.phone_number?.area_code
        
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

    #[test]
    fn get_person_job_area_code() {
        let p = Person {
            job: Some(Job {
                phone_number: Some(PhoneNumber {
                    area_code: Some(61),
                    number: 439222222,
                }),
            }),
        };
    
        assert_eq!(p.work_phone_area_code(), Some(61));
    }
}
