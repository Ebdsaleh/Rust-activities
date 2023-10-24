// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

pub fn main() {

    
    print_colour_name(&Colours::Blue);




}

pub enum Colours {
    Black,
    Red,
    Green,
    Blue,
    Pink,
    Yellow,
    
}
pub fn print_colour_name(colours: &Colours) {
    let message: &str;


    match colours {

        Colours::Black => message = "Black",
        Colours::Red => message = "Red",
        Colours::Green => message = "Green",
        Colours::Blue => message = "Blue",
        Colours::Pink => message = "Pink",
        Colours::Yellow => message = "Yellow",
    }
    println!("{}", message);
}
