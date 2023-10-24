// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

pub fn main() {


    let value: i32 = 101;

    
    let result: bool = value > 100;
    print_result(result);
 }


pub fn print_result(result: bool) {
    let msg: &str;
    match result{
        true => msg = "it's big",
        false => msg = "it's small",
    }
    println!("{}", msg);



} 
