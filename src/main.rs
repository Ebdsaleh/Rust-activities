use std::io;
use activities::{ a1, a2, a3a, a3b, a4a, a4b, a5, a6, a7, a8, a9,
                    a10, a11, a12, a13, a14, a15, a16, a17,a18, a18b,a19, terminal_helper};



pub fn main() {
    // set the activity files array
    // creates and array of i32's from 0 to 18
    let mut activities:Vec<String> = Vec::with_capacity(22); 
    for i in 0..22 {
        activities.push(i.to_string());
    }


    // Consider using a while loop 
    loop {
        // Clear the screen
        terminal_helper::clear_screen();
        // Get input from the user to select which ativity to run
        let mut input: String = String::new();
        let mut activity_found = false;
        println!("Welcome to the Rust activity runner! "); 
        println!("Type a number between (0 - 21) to select an activity to run, or type 'quit' to exit:"); 
        println!("Which activity do you want to execute (0 - 21):"); 
        io::stdin().read_line( &mut input).unwrap();
        input = input.trim().to_string();
        // if input is "quit" then exit
        if input == "quit".to_string() { 
            println!("Thank you, come again!");
            return;}
        println!("You have selected to run activity: {input} ");
        for (i, v) in  activities.iter().enumerate() {
            if v == &input {
                println!("Activity found!");
                activity_found = true;
                break; 
            }        
            if i   == activities.len() - 1 && v != &input {
                println!("Error : Activity {input}, not found!");
            }


        }
        if activity_found {
            // run main function from activity file.
            println!("Loading activity {input}");
            load_acitivity(&input);
        } else { 
            println!("No valid activity found, try again.");
            terminal_helper::wait_for_input();
            continue;
        }
    }
}

pub fn load_acitivity(activity: &String) {
    if activity == "0" { a1::main(); terminal_helper::wait_for_input();}
    if activity == "1" { a2::main(); terminal_helper::wait_for_input();}
    if activity == "2" { a3a::main(); terminal_helper::wait_for_input();}
    if activity == "3" { a3b::main(); terminal_helper::wait_for_input();}
    if activity == "4" { a4a::main(); terminal_helper::wait_for_input();}
    if activity == "5" { a4b::main(); terminal_helper::wait_for_input();}
    if activity == "6" { a5::main(); terminal_helper::wait_for_input();}
    if activity == "7" { a6::main(); terminal_helper::wait_for_input();}
    if activity == "8" { a7::main(); terminal_helper::wait_for_input();}
    if activity == "9" { a8::main(); terminal_helper::wait_for_input();}
    if activity == "10" { a9::main(); terminal_helper::wait_for_input();}
    if activity == "11" { a10::main(); terminal_helper::wait_for_input();}
    if activity == "12" { a11::main(); terminal_helper::wait_for_input();}
    if activity == "13" { a12::main(); terminal_helper::wait_for_input();}
    if activity == "14" { a13::main(); terminal_helper::wait_for_input();}
    if activity == "15" { a14::main(); terminal_helper::wait_for_input();}
    if activity == "16" { a15::main(); terminal_helper::wait_for_input();}
    if activity == "17" { a16::main(); terminal_helper::wait_for_input();}
    if activity == "18" { a17::main(); terminal_helper::wait_for_input();}
    if activity == "19" { a18::main(); terminal_helper::wait_for_input();}
    if activity == "20" { a18b::main(); terminal_helper::wait_for_input();}
    if activity == "21" { a19::main(); terminal_helper::wait_for_input();}
}

