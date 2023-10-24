// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

pub fn main() {
    
    let bananas = GroceryItem::new(101, "Bananas", 10);
    println!("{} Item Information", bananas.name);
    print_quantity(&bananas);
    print_id(&bananas);

}


pub struct GroceryItem<'name> {
    id: i32,
    name: &'name str,
    quantity: i32,

}

impl<'name> GroceryItem<'name>{
    pub fn new(id: i32, name: &'name str, quantity: i32) -> Self {
        Self {
            id,
            name,
            quantity,
        }
    }
}

pub fn print_quantity(item: &GroceryItem) {
    println!("item ID: {}", item.quantity);
}

pub fn print_id(item: &GroceryItem) {
    println!("Quantity: {}", item.id);

}   
// not required but added for completeness.
pub fn print_name(item: &GroceryItem) {
    println!("Name: {}", item.name);
}
