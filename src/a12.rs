// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

pub fn main() {

    let shipping_box_0 = ShippingContainer::new(Dimensions::new(3,10,3), 2000, Color::Red);
    let shipping_box_1 = ShippingContainer::new(Dimensions::new(4,12, 4), 2500, Color::Green);
    let shipping_box_2 = ShippingContainer::new(Dimensions::new(4,15, 4), 5000, Color::Blue);
    shipping_box_0.print_info();
    shipping_box_1.print_info();
    shipping_box_2.print_info();
    }


struct ShippingContainer {
    dimensions: Dimensions,
    weight: i32,
    color: Color,
}

struct Dimensions {
    width:i32,
    height:i32,
    depth:i32,
}

enum Color {
    Red,
    Green,
    Blue,

}
impl Dimensions {
    fn new(width: i32, height:i32, depth:i32) -> Self {
        Self {
            width,
            height,
            depth,
        }
    }
}
impl ShippingContainer {

    fn new(dimensions: Dimensions, weight: i32, color: Color) -> Self {
        Self{
            dimensions,
            weight,
            color,
        }
    }
    fn print_info (&self) {
        let color: &str;
        match self.color {
            Color::Red => color = "Red",
            Color::Green => color = "Green",
            Color::Blue=> color = "Blue",
        }
        println!("\nShipping Container Information:");
        println!("width: {}\nheight: {}\ndepth: {}\nweight: {}\ncolor: {}\n", 
            self.dimensions.width, self.dimensions.height, self.dimensions.depth,self.weight, color);

    }
}
