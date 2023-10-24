use std::{io, env};

use crate::{ LINUX, WINDOWS };




pub fn clear_screen() {
    if env::consts::OS == LINUX.to_string(){
        std::process::Command::new("clear").status().unwrap();
    }
    println!("You are running on {}!", env::consts::OS); // Prints the current OS.

    if env::consts::OS == WINDOWS.to_string() {
        std::process::Command::new("cls").status().unwrap();
    }
}

pub fn wait_for_input() {

    println!("Press 'enter' to continue");
    
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
}
