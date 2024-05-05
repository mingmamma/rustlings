// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.

struct Book<'a> {
    author: &'a str,
    title: &'a str,
    formal_name: &'a mut String
}

impl<'a> Book<'a> {
    // noting this is a "static method?!" that doesnot involve an instance of Book struct, not any other references in any form
    // apart from the apperent string literal, thus no lifetime annotation is involved whatsoever
    fn struct_generic_info() {
        println!("Just books")
    }

    // The following two should demonstrate a lifetime annotation elision of the first rule, i.e.
    // a lifetime parameter is assigned to each parameter that is a reference in the functions paramater list
    // this is applicable to the only parameter in the method, which is a shared reference to the struct instance
    fn instance_info_elisioned(&self) -> () {
        println!("This is a book, {} by {}", self.title, self.author)
    }

    // no constraint checked by the annotatation in impl declaration `impl <'a> Book <'a>` 
    // and method siginature `fn ...<'b>(self: &'b Book<'a>)`
    fn instance_info_original<'b>(self: &'b Book<'a>) -> () {
    // fn instance_info_original<'b>(&'b self) -> () {
        println!("This is a book, {} by {}", self.title, self.author)
    }

    // the following two is nothing different from the preceding two, except that the parameter in the method
    // whose explicit annotation is elisioned is a mutable reference to the struct instance but not a shared one
    fn instance_formal_info_original(&mut self) {
        self.formal_name.push_str(" ,an acclaimed author");
        println!("This is a great book, {} by {}", self.title, self.formal_name)
    }

    // fn instance_formal_info_original<'b>(&'b mut self) {
    fn instance_formal_info_elisioned<'b>(self: &'b mut Book<'a>) {
        self.formal_name.push_str(" ,an acclaimed author");
        println!("This is a great book, {} by {}", self.title, self.formal_name)
    }
}

// Illustrative example from lecture: Crust of Rust, Lifetime Annotations, https://www.youtube.com/watch?v=rAl-9HwD858
// Following implementation in std library: 
struct StrSplit<'a, 'b> {
    remainder: Option<&'a str>,
    delimiter: &'b str,
}

impl<'a, 'b> StrSplit<'a, 'b> {
    fn new(haystack: &'a str, needle: &'b str) -> StrSplit<'a, 'b> {
        if needle.is_empty() {
            panic!("expect the delimiter to be non empty string")
        };
        StrSplit {
            remainder: Some(haystack),
            delimiter: needle,
        }
    }
}

impl<'a, 'b> Iterator for StrSplit<'a, 'b> {
    type Item = &'a str;
    // fn next(self: &'a mut Self) -> Option<Self::Item> {
    // fn next(self: &'a mut Self) -> Option<<Self as Iterator>::Item> {
    // fn next(self: &'a mut StrSplit<'a>) -> Option<Self::Item> { /* what does the explicit lifetime annotation of the parameter in the next method mean?! */
    
    // fn next<'c>(self: &'c mut StrSplit<'a, 'b>) -> Option<&'a str> {
    //     match self.remainder {
    //         Some(input_string) => {
    //             match input_string.find(self.delimiter) {
    //                 Some(delimiter_match_index) => {
    //                     // Update the remainder s.t. the next evaluation of the iterator would consider the portion of string
    //                     // immediately after the occurance of the found delimiter
    //                     self.remainder = Some(&input_string[(delimiter_match_index+self.delimiter.len()) .. input_string.len()]);
                        
    //                     // return the result containing the portion of the string immediately before the occurance of the found delimiter
                        
    //                     // index syntax on str type enabled by these trait implementations:
    //                     // https://doc.rust-lang.org/std/primitive.str.html#impl-Index%3CI%3E-for-str, enables index syntax `[ ??? ]` for &str type
    //                     // https://doc.rust-lang.org/std/primitive.str.html#impl-SliceIndex%3Cstr%3E-for-Range%3Cusize%3E, enables using the inclusive range struct
    //                     // `start .. end` within the index syntax `[ ??? ]`
    //                     Some(&input_string[0 .. delimiter_match_index])
    //                 },
    //                 None => {
    //                     // take() returns the Some wrapped str as is and transitions the field to None variant s.t. further evaluation of 
    //                     // the iterator would return None based on the field of None
    //                     self.remainder.take()
    //                 }                    
    //             }
    //         }
    //         None => None
    //     }
    // }

    fn next(&mut self) -> Option<&'a str> {
        // obtain a mutable reference into the Option wrapped value with as_mut
        // use ? operator does NOT affect the ownership of the Option value on which it is applied on?!
        // the ? operator on Option returns the value wrapped in Some, or returns None early back to the enclosing function
        // https://doc.rust-lang.org/std/option/index.html#the-question-mark-operator-
        // how to reason that the variable assigned with the mutable reference of type &mut &str, the pointee of which is a value
        // of type &str, is a value which is INDEED owned by the original Option and thus the original struct?!
        // the diciding factor of how the ownership of the value in assignment/match scrutinee/function argument is affected, or
        // otherwise reference of the value are created, is a matter mostly concerning the pattern, and of course concerning if
        // the value is Copy. How to use that to reason with fix of `if let Some(ref mut ...) = op_val` to `if let Some(..) = op_val`
        // in https://www.youtube.com/watch?v=rAl-9HwD858&t=3339s
        // https://users.rust-lang.org/t/match-dereference/91352/2
        
        // Consider the following incorrect implementation. Due to &str being a type implementing the Copy trait,
        // and in fact any shared reference value &T implementing the Copy trait, the assignment to str_of_remainder
        // copies rather than move ownership, implying that str_of_remainder variable, eventually, is a mutable reference whose pointee is NOT the &str value owned by
        // the self.remainder Option, and by extension, owned by the self StrSplit struct. It points to be separate &str value caused by the copy. This, in turns, leads
        // to updating the pointee via dereferencing ineffective: *str_of_remainer = ...
        // let mut str_of_remainder = self.remainder?;
        // let str_of_remainder = &mut str_of_remainder;
        // Equivalent to one-liner with ref mut
        // let ref mut str_of_remainder = self.remainder?;

        // if let Some(ref mut remainder) = self.remainder
        let ref mut remainder = self.remainder?; 

        match self.remainder.as_mut() {
            None => None,
            Some(str_of_remainder) => {
                // reasoning of the validity of application of str methods on a value of type &mut &str:
                // Noting that the Deref trait contract: https://doc.rust-lang.org/std/ops/trait.Deref.html
                // pub trait Deref {
                //      type Target: ?Sized;
                //      fn deref(&self) -> &Self::Target;}
                // Given the blanket implementation: https://doc.rust-lang.org/std/ops/trait.Deref.html#impl-Deref-for-%26mut+T
                // impl Deref<Target = T> for &mut T
                // it is entailed that &mut &str implicitly implement all the methods of &str that takes a &self receiver
                // by the last proposition of the implication of Deref Coersion: https://doc.rust-lang.org/std/ops/trait.Deref.html#deref-coercion
                // Applying similar arguement on another blanket implementation:
                // impl Deref<Target = T> for &T https://doc.rust-lang.org/std/ops/trait.Deref.html#impl-Deref-for-%26T
                // We see further that &str implement all methods of str that takes a &self receiver, which are the majority of method documented in:
                // https://doc.rust-lang.org/std/primitive.str.html#&mut. Hence &mut &str have all methods of str taking a &self receiver
                if let Some(delimiter_match_index) = str_of_remainder.find(self.delimiter) {

                    // the addition & before the container[index] syntax should be due to the need to
                    // disambiguate between desugaring to a index() or a index_mut() method call of str
                    let str_till_delimiter = &str_of_remainder[..delimiter_match_index];
        
                    // update the value within the wrapped Option in place, by accessing it through dereferencing the mutable pointer to it
                    *str_of_remainder = &str_of_remainder[delimiter_match_index+self.delimiter.len()..];
                    Some(str_till_delimiter)
                } else {
                    self.remainder.take()
                }
            }
        }
    }
}

// extented functionality to return the portion of a string preceding the given delimiter should the delimiter
// is found in the input string, or the whole input string otherwise, with delimiter of a char type
fn split_by_char<'a>(input_str: &'a str, delimiter: char) -> &'a str {
    let _delimter_str = String::from(delimiter);
    let mut _str_split = StrSplit::new(input_str, &_delimter_str);
    // precise lifetime annotation on StrSplit struct is needed s.t. the compiler could correctly perform borrow check
    // with the view that the lifetime of the delimiter &str is NOT tied to the returned Option<&str> of evaluating the iterator
    _str_split.next()
              .expect("the Str Split iterator used for implementation should always yield a Some variant on the first evaluation")
}

// further extension functionality to work with any type for the delimiter parameter, given value of that type can represent the delimiter in the input string
// to that end, instead of considering converting types to &str and to fits the converted &str delimiter the existing StrSplit struct,
// the better pattern is to design a trait for representing the class of delimiter that has the ability to reports its location (if present)
// in a given &str parameter (serving the role of input string to be split). Such a pattern inverts the roles of finding delimiter in input string
// s.t evaluating the StrSplit iterator relegates the main delimiter search implementation to this new pattern
// This pattern is used in indexing of &str: 

// fn split_by_delim<D, 'a>(input_str:&'a str, delimiter: D) -> &'a str {}

struct StrSplit2<'a, D: Delimiter> {
    remainder: Option<&'a str>,
    delimiter: D,
}

impl<'a, D: Delimiter> StrSplit2<'a, D> {
    fn new(input_str: &'a str, delimiter: D) -> Self {
        Self {
            remainder: Some(input_str),
            delimiter,
        }
    }
}

impl<'a, D: Delimiter> Iterator for StrSplit2<'a, D> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        match self.remainder {
            Some(input_str) => {
                match self.delimiter.get_delimit_indeces(input_str) {
                    Some((start_index, end_index)) => {
                        self.remainder = Some(&input_str[end_index..]);
                        Some(&input_str[..start_index])
                    },
                    None => self.remainder.take(),
                }
            }
            None => None
        }
    }
}

trait Delimiter {
    // the specification of the ability for the trait is the ability expressed by the 
    // following function signature with which the start and the end index of the first
    // occurance of the delimiter within the input string can be returned, should the delimiter
    // be present in the first place. The return type is modelled with Option s.t. None is for
    // when the delimiter is NOT in the str input at all
    fn get_delimit_indeces(&self, input_str: &str) -> Option<(usize, usize)>;
}

impl Delimiter for char {
    fn get_delimit_indeces(&self, input_str: &str) -> Option<(usize, usize)> { 
        // find() method of str can take char directly as parameter
        // match input_str.find(*self) {
        //     Some(delim_match_index) => Some((delim_match_index, delim_match_index + self.len_utf8())),
        //     None => None,
        // }
        
        // alternative approach than using find() method of str directly is to work with iterators of str, of which the more usable two APIs are 
        // chars() and char_indicies() for an iterator involving chars from the str, and bytes() for an iterator of bytes of the str
        // then the find() method inherent to the Iterator trait gets used in place of find() of str
        input_str.char_indices()
                 .find(|char_index_tuple: &(usize, char)| -> bool {
                    (*char_index_tuple).1 == *self
                 })
                 .map(|located_char_index: (usize, char)| -> (usize, usize) {
                    (located_char_index.0, located_char_index.0 + self.len_utf8())
                 })
    }
}

impl Delimiter for &str {
    fn get_delimit_indeces(&self, input_str: &str) -> Option<(usize, usize)> {

        // use map for handling Some/None variants processing rather than using match
        input_str.find(self)
                 .map(|delim_match_index: usize| -> (usize, usize) {(delim_match_index, delim_match_index + self.len())})
    }
}

fn split_by_char_2(input_str: &str, delimiter: char) -> &str {
    let delim = format!("{delimiter}");
    // let mut _str_split = StrSplit2::new(input_str, &*delim); /* trick to go from String to &str */
    let mut str_split = StrSplit2::new(input_str, delim.as_ref()); /* probably a bit more proper than the trick */
    
    str_split.next()
             .expect("the Str Split iterator used for implementation should always yield a Some variant on the first evaluation")
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let mut formal_name = String::from("Jill Smith");

    Book::struct_generic_info();
    
    let mut book = Book { author: &name, title: &title, formal_name: &mut formal_name};
    println!("{} by {}", book.title, book.author);
    book.instance_info_elisioned();
    book.instance_info_original();
    book.instance_formal_info_elisioned();
    book.instance_formal_info_original();

    let mut str_split_instance = StrSplit::new("a,b", ",");
    assert_eq!(str_split_instance.next(), Some("a"));
    assert_eq!(str_split_instance.next(), Some("b"));
    assert_eq!(str_split_instance.next(), None);

    let mut str_split_instance_2 = StrSplit::new("abc", ",");
    assert_eq!(str_split_instance_2.next(), Some("abc"));
    assert_eq!(str_split_instance_2.next(), None);

    // the full specific of the iterator functionality can be described as
    // given a input of the Itetator consisting of an input string to iterate over and a delimiter also of a string
    // the result returned by the iterator is the substring of the input string which immediately proceeds the first occurance of the delimiter
    // in the case that the delimiter is contained in the input string to be considered. The evaluation of the iterator should return the result
    // evaluated on the portion of string immediately following the delimiter considered as the input string with the same delimiter following that
    // case. In case the delimiter is NOT present in the input string to be evaluated in the first place, the whole input string should be returned
    // as result of evaluation of the iterator, and subsequent evaludation of the iterator should yield None.

    // Given such specification, modelling the iterator's input string with a &str type would not be sufficent since it can not distinguish the nuance of whether
    // a empty string or a None should be the result of evaluating the iterator when the input string is an empty string. The result depends on whether the delimiter
    // was in the input string in the previous evaluation, the information of which cannot be expressed by modelling the input string with a &str type.
    // Modelling the input string with Option<&str> can work around the issue by the additional semantic given by the wrapping Option.
    let mut str_split_instance_3 = StrSplit::new("a,", ",");
    assert_eq!(str_split_instance_3.next(), Some("a"));
    assert_eq!(str_split_instance_3.next(), Some(""));
    assert_eq!(str_split_instance_3.next(), None);

    assert_eq!(split_by_char("a,b,c", ','), "a");
    assert_eq!(split_by_char("abc", ','), "abc");
    assert_eq!(split_by_char(",", ','), "");
    assert_eq!(split_by_char("", ','), "");

    let mut str_split_2_instance = StrSplit2::new("a,b", ',');
    assert_eq!(str_split_2_instance.next(), Some("a"));
    assert_eq!(str_split_2_instance.next(), Some("b"));
    assert_eq!(str_split_2_instance.next(), None);

    let mut str_split_2_instance_2 = StrSplit2::new("abc", ',');
    assert_eq!(str_split_2_instance_2.next(), Some("abc"));
    assert_eq!(str_split_2_instance_2.next(), None);

    let mut str_split_2_instance_3 = StrSplit2::new("a,", ',');
    assert_eq!(str_split_2_instance_3.next(), Some("a"));
    assert_eq!(str_split_2_instance_3.next(), Some(""));
    assert_eq!(str_split_2_instance_3.next(), None);

    let mut str_split_2_instance_4 = StrSplit2::new("a,b", ",");
    assert_eq!(str_split_2_instance_4.next(), Some("a"));
    assert_eq!(str_split_2_instance_4.next(), Some("b"));
    assert_eq!(str_split_2_instance_4.next(), None);

    let mut str_split_2_instance_5 = StrSplit2::new("abc", ",");
    assert_eq!(str_split_2_instance_5.next(), Some("abc"));
    assert_eq!(str_split_2_instance_5.next(), None);

    let mut str_split_2_instance_6 = StrSplit2::new("a,", ",");
    assert_eq!(str_split_2_instance_6.next(), Some("a"));
    assert_eq!(str_split_2_instance_6.next(), Some(""));
    assert_eq!(str_split_2_instance_6.next(), None);

    assert_eq!(split_by_char_2("a,b,c", ','), "a");
    assert_eq!(split_by_char_2("abc", ','), "abc");
    assert_eq!(split_by_char_2(",", ','), "");
    assert_eq!(split_by_char_2("", ','), "");
}
