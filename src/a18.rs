// Topic: Result
//
// Requirements:
// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using `derive`
// * Implement a `new` function for the `Adult` structure that returns a Result:
//   * The Ok variant should contain the initialized structure, but only
//     if the person is aged 21 or older
//   * The Err variant should contain a String (or &str) that explains why
//     the structure could not be created
// * Instantiate two `Adult` structures:
//   * One should be aged under 21
//   * One should be 21 or over
// * Use `match` to print out a message for each `Adult`:
//   * For the Ok variant, print any message you want
//   * For the Err variant, print out the error message

pub fn main() {
   let over_21 = Adult::new(38, "Nadir");
   let under_21 =  Adult::new(8, "Krystal");

   match over_21 {
       Ok(adult) => println!("{:?} is {:?} years old.", adult.name, adult.age),
       Err(e) => println!("{:?}", e),
   }
    
   match under_21 {
       Ok(child) => println!("{:?} is {:?} years old.", child.name, child.age),
       Err(e) => println!("{:?}", e),
   }

}

#[derive(Debug)]
struct Adult{
    age: u8,
    name: String,
}

impl Adult {

    fn new(age: u8, name: &str) -> Result<Self, &str> {

        if age >= 21 {
            Ok(Self {
                age,
                name: name.to_string()
            })
        } else {
            Err("Age must be higher than 20!")
        }
    }
}
