// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

pub fn main() {

    let mut people: Vec<Person> = Vec::new();
    people.push(Person::new("Nadir", 38, "blue"));
    people.push(Person::new("Anne", 71, "purple"));
    people.push(Person::new("Krystal", 8, "pink"));
    people.push(Person::new("Juliva", 35, "pink"));
    people.push(Person::new("Bianca", 7, "green"));
    people.push(Person::new("Aeron", 6, "blue"));
    people.push(Person::new("Keisha", 9, "blue"));
    people.push(Person::new("Joe", 10, "yellow"));
    people.push(Person::new("Adam", 1, "red"));
    people.push(Person::new("Mike", 4, "orange"));
    
    for (i, v) in people.iter().enumerate() {
        if people[i].age < 10 {
            v.print_info();
        } else { 
            continue;
        }
    }


}

struct Person {
    name:  String,
    age: i32,
    color: String,
}

impl Person {
   fn new(name: &str, age: i32, color: &str) -> Self {
        Self {
            name: name.to_owned(),
            age,
            color: color.to_owned(), 
        }
    }
   fn print_info(&self){
       println!("\nname: {}\ncolor: {}\n",self.name, self.color);
   }
}
