// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

pub fn main() {

    let bottle_of_cola = Drink {flavour: Flavour::Cola, volume: 80};
    let bottle_of_orange = Drink::new(Flavour::Orange, 120);

    print_drink_flavour(&bottle_of_cola);
    bottle_of_orange.print_info();
    print_drink_flavour(&Drink{flavour: Flavour::Lemon, volume: 30});


}

pub enum Flavour {
   Cola,
   Orange,
   Lemon,
}

pub struct Drink {
    flavour: Flavour,
    volume: i32,
}

impl Drink {

    pub fn new(flav: Flavour, vol: i32) -> Self {

        Self {
            flavour: flav,
            volume: vol,
        }
    }
    pub fn get_flavour(&self) -> &str {

        match self.flavour {
            Flavour::Cola => "Cola",
            Flavour::Orange => "Orange",
            Flavour::Lemon => "Lemon",
        }
    }   

    pub fn get_volume(&self) -> i32 {
        self.volume
    }

    pub fn print_info(&self) {
        // This next match statement is unesscessary but I'm using this to meet the activity
        // requirements
        match self.flavour {
            Flavour::Cola => println!("drink flavour is: {} and is {} oz in fluid volume.", self.get_flavour(), self.get_volume()),
            Flavour::Orange => println!("drink flavour is: {} and is {} oz in fluid volume.", self.get_flavour(), self.get_volume()),
            Flavour::Lemon => println!("drink flavour is: {} and is {} oz in fluid volume.", self.get_flavour(), self.get_volume()),
        }
    }
}

pub fn print_drink_flavour(drink: &Drink) {

    let flavour: &str = drink.get_flavour();
    let volume: i32 =  drink.get_volume();
    println!("drink flavour is: {} and is {} oz in fluid volume.", flavour, volume);
}
