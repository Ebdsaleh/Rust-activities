// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

pub fn main() {
    let num1 = 20;
    let num2 = 22;
    display_result(&sum(&num1, &num2));
}


pub fn sum(num1: &i32, num2: &i32) -> i32 {
    num1 + num2

}

pub fn display_result(result: &i32) {
    println!("result: {:?}", result);
}
