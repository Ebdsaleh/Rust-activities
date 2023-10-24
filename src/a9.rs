// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print


pub fn main() {


    let (x, y) = return_coords();
    let comp_value: i32 = 5;
    let comp_result: &str;


    if y > comp_value {
        comp_result = "greater than";
    } else if y < comp_value {
        comp_result = "less than";
    } else {
        comp_result = " equal to";
    }
    println!("Coordinates are x: {}, y: {}", x, y);
    println!("y: {} is {} {}", y, comp_result, comp_value);


}

pub fn return_coords() -> (i32, i32) {
    (10, 4)
}

