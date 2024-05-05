use std::num::ParseIntError;
use std::result;
use std::error;
use std::fmt;

fn double_first(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt_parsed_first_ele: Option<Result<i32, ParseIntError>> = vec.first().map(|first: &&str| -> Result<i32, ParseIntError> {
        first.parse::<i32>().map(|n| 2 * n)
    });
    
    
    opt_parsed_first_ele.map_or(Ok(None), |res: Result<i32, ParseIntError>| -> 
        Result<Option<i32>, ParseIntError> {res.map(|r: i32| -> 
            Option<i32> {Some(r)}
        )}
    )
}

// Define a new type for custom Error and a type alias of Result to work with the Error
#[derive(Debug)]
enum DoubleError2 {
    EmptyVecError,
    ParseIntError,
}

type CustomResult<T> = result::Result<T, DoubleError2>;

fn double_first_2(vec: Vec<&str>) -> CustomResult<i32> {
    let vec_first_result: CustomResult<&&str> = vec.first().ok_or(DoubleError2::EmptyVecError);
    
    vec_first_result.and_then(|first_str: &&str| -> CustomResult<i32> {
            first_str.parse::<i32>()
                .map_err(|_| -> DoubleError2 {DoubleError2::ParseIntError {}})
                .map(|i: i32| -> i32 {2 * i})
        })
}

impl fmt::Display for DoubleError2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Empty vector to be doubled")
    }
}

// the requirement to implement std::error::Error trait for a type equates to the type implementing both the Debug and Display traits
// https://doc.rust-lang.org/std/error/trait.Error.html
impl error::Error for DoubleError2 {}

// Another approach to consolidate different types of error without explicitly modelling them is to box the errors
fn double_first_3(vec: Vec<&str>) -> Result<i32, Box<dyn error::Error>> {
    // converting the custom error type in to a boxed error type requires the custom error type to properly implement the std::error::Error trait
    // https://doc.rust-lang.org/std/boxed/struct.Box.html#impl-From%3CE%3E-for-Box%3Cdyn+Error%3E
    let vec_first_result: Result<&&str, Box<dyn error::Error>> = vec.first()
                                                                    .ok_or_else(|| -> Box<dyn error::Error>
                                                                        {DoubleError2::EmptyVecError.into()}
                                                                    );
    
    // the order of application of map and map_err functions is commutative since they operate on disjoint
    // variants on the Result enum
    vec_first_result.and_then(|first_str: &&str| -> Result<i32, Box<dyn error::Error>> {
        first_str.parse::<i32>()
                 .map(|i| 2 * i)
                 .map_err(|_| -> Box<dyn error::Error> {DoubleError2::ParseIntError.into()})
    })
}

// An example exhibiting the more subtle effect of the ? operator
fn double_first_4(vec: Vec<&str>) -> Result<i32, Box<dyn error::Error>> {
    
    // the following shows that beyond return the Err variant early i.e. return Err(err), the ? operator also has the implicit effect
    // of facilitating the type conversion in the context of Box'd error, i.e. return Err(Box::from(err))
    let first = vec.first().ok_or(DoubleError2::EmptyVecError)?;
    let parsed = first.parse::<i32>().map_err(|_| DoubleError2::ParseIntError)?;
    
    Ok(2 * parsed)
}


fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];
    
    println!("The first doubled is {:?}", double_first(numbers.clone()));
    
    // Ok(None)
    println!("The first doubled is {:?}", double_first(empty.clone()));
    
    // Err(ParseIntError)
    println!("The first doubled is {:?}", double_first(strings.clone()));
    
    println!("The first doubled is {:?}", double_first_2(numbers.clone()));
    
    // Err(EmptyVecError)
    println!("The first doubled is {:?}", double_first_2(empty.clone()));
    
    // Err(ParseIntError)
    println!("The first doubled is {:?}", double_first_2(strings.clone()));
    
    println!("The first doubled is {:?}", double_first_3(numbers.clone()));
    
    // Err(EmptyVecError), the debug print treats the wrapping box transparently
    println!("The first doubled is {:?}", double_first_3(empty.clone()));
    
    // Err(ParseIntError), ditto
    println!("The first doubled is {:?}", double_first_3(strings.clone()));
    
    println!("The first doubled is {:?}", double_first_4(numbers.clone()));
    
    // Err(EmptyVecError)
    println!("The first doubled is {:?}", double_first_4(empty.clone()));
    
    // Err(ParseIntError)
    println!("The first doubled is {:?}", double_first_4(strings.clone()));
}