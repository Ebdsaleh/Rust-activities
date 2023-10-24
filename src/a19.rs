// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

pub fn main() {
    let mut funiture: HashMap<&str, i32> = HashMap::new();
    funiture.insert("Chairs", 5);
    funiture.insert("Beds", 3);
    funiture.insert("Tables", 2);
    funiture.insert("Couches", 0);
    let mut total_items: i32 = 0; 
    println!("\n\nThe store has:\n");
    for (item_name, quantity) in funiture.iter() {
        
        if *quantity > 0 {
            total_items += quantity;
         println!("{} {} in stock.", quantity, item_name);
        }
        else {
            println!("{} are out of stock ", item_name);
        }
    }
    println!("\nThere are {} total items in stock", total_items);





}
