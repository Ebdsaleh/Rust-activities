// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively
//
// Notes:
// * Use a variable set to any integer
// * Use a match expression to determine which message to display
// * Use an underscore (_) to match on any value

pub fn main() {

    let value:i32 = 2;
    let message: &str;
    match value {
        1 => message = "one",
        2 => message = "two",
        3 => message = "three",
        _=> message = "other",

    }

    println!("{}", message);

}
