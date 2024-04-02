// errors1.rs
//
// This function refuses to generate text to be printed on a nametag if you pass
// it an empty string. It'd be nicer if it explained what the problem was,
// instead of just sometimes returning `None`. Thankfully, Rust has a similar
// construct to `Option` that can be used to express error conditions. Let's use it!
//
// Execute `rustlings hint errors1` or use the `hint` watch subcommand for a
// hint.

// lifetime annomation 'static is required in this case?!
// lifetime annotation is need on a reference in the function signature, if not elisioned?!
pub fn generate_nametag_text(name: String) -> Result<String, &'static str> {
    if name.is_empty() {
        // Empty names aren't allowed.
        // Err(String::from("`name` was empty; it must be nonempty."))
        Err("`name` was empty; it must be nonempty.")
    } else {
        Ok(format!("Hi! My name is {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            // Presumably, the into() call converts a &str to String, as expected by the generate_nametag_text method ?!
            // We could use into() call on a &str entails the existance of an implementation of Into<String> for &str
            // s.t. the required method type checks: fn into(self: &str) -> String, see: https://doc.rust-lang.org/std/convert/trait.Into.html
            // As pointed out by the relation between From and Into trait, Into<String> for &str is implied by the implementation of From<&str> for String
            // which would provide its Into dual by the machemism of blanket implementation, see: https://doc.rust-lang.org/std/convert/trait.From.html
            // The overall assumption checks out since we can fine From<&str> for String from the std lib: https://doc.rust-lang.org/std/string/struct.String.html#impl-From%3C%26str%3E-for-String 
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // change the original line to cater for Result<String, &str> in place of Result<String, String>
            // Err("`name` was empty; it must be nonempty.".into())
            Err("`name` was empty; it must be nonempty.")
        );
    }
}
