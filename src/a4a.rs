// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display

pub fn main() {
    let value: bool = true;
    let message: &str;

    match value {
        true => message = "it's true",
        false => message = "it's false",
    }
    
    println!("{}", message);



}
