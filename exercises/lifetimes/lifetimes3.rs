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


struct StrSplit<'a> {
    remainder: Option<&'a str>,
    delimiter: &'a str,
}

impl<'a> StrSplit<'a> {
    fn new(haystack: &'a str, needle: &'a str) -> StrSplit<'a> {
        if needle.is_empty() {
            panic!("expect the delimiter to be non empty string")
        };
        StrSplit {
            remainder: Some(haystack),
            delimiter: needle,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;
    // fn next(self: &'a mut Self) -> Option<Self::Item> {
    // fn next(self: &'a mut Self) -> Option<<Self as Iterator>::Item> {
    // fn next(self: &'a mut StrSplit<'a>) -> Option<Self::Item> { /* what does the explicit lifetime annotation of the parameter in the next method mean?! */
    
    // this signature captures the lifetime constraint that really matters?! 
    fn next<'b>(self: &'b mut StrSplit<'a>) -> Option<&'a str> {
        match self.remainder {
            Some(input_string) => {
                match input_string.find(self.delimiter) {
                    Some(delimiter_match_index) => {
                        // Update the remainder s.t. the next evaluation of the iterator would consider the portion of string
                        // immediately after the occurance of the found delimiter
                        self.remainder = Some(&input_string[(delimiter_match_index+self.delimiter.len()) .. input_string.len()]);
                        
                        // return the result containing the portion of the string immediately before the occurance of the found delimiter
                        
                        // index syntax on str type enabled by these trait implementations:
                        // https://doc.rust-lang.org/std/primitive.str.html#impl-Index%3CI%3E-for-str, enables index syntax `[ ??? ]` for &str type
                        // https://doc.rust-lang.org/std/primitive.str.html#impl-SliceIndex%3Cstr%3E-for-Range%3Cusize%3E, enables using the inclusive range struct
                        // `start .. end` within the index syntax `[ ??? ]`
                        Some(&input_string[0 .. delimiter_match_index])
                    },
                    None => {
                        // take() returns the Some wrapped str as is and transitions the field to None variant s.t. further evaluation of 
                        // the iterator would return None based on the field of None
                        self.remainder.take()
                    }                    
                }
            }
            None => None
        }
    }
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

    let mut str_split_instance = StrSplit::new("a,b,c", ",");
    assert_eq!(str_split_instance.next(), Some("a"));
    assert_eq!(str_split_instance.next(), Some("b"));
    assert_eq!(str_split_instance.next(), Some("c"));
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
    // let mut str_split_instance_3 = StrSplit("a,b,", ",");
    // assert_eq!(str_split_instance_3.next(), Some("a"));
    // assert_eq!(str_split_instance_3.next(), Some("b"));
    // assert_eq!(str_split_instance_3.next(), Some(""));
    // assert_eq!(str_split_instance_3.next(), None);

}
