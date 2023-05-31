use std::{io::{self, Write}, process};
use termion::color::{self, Red, Blue, Green, Yellow};

/// input: creates a String and then calls io::stdin().read_line to read input for the string.
/// 
/// s: What to print out before asking for input.
pub fn input(s: &str)-> String {
    let mut val = String::new();
    
    eprint!("{}", s);
    io::stdin().read_line(&mut val).expect("Could not read input.");

    return val;
}
/// exec: creates a Command with the cmd and arg parameters and returns the output.
/// 
/// cmd: The command to run.
/// arg: The command argument.
pub fn exec(cmd: &str, arg: &str) -> process::Output {
    let output = process::Command::new(cmd).arg(arg).output().expect("Could not run system command");

    return output;
}
/// number_vec: create a vector with a specific amount of numbers.
/// 
/// n: the amount of numbers for this vector.
pub fn number_vec(n:i32) -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::new();

    for i in 0..n {
        vec.push(i);
    }

    vec.push(n);

    return vec;
}

/// curl: run a curl and print the output to stdout.
/// 
/// url: the URL to connect to.
pub fn curl(url:&str) {
    let output = exec("curl", url);
    io::stdout().write_all(&output.stdout).unwrap();
}


/// print_color: print text but with color
/// 
/// s: The string to print out.
/// c: The color, either: ["red", "blue", "green", or "yellow"]
pub fn print_color(s: &str, c: &str) {
    let  col = String::from(c);
    
    match col.to_lowercase().trim() {
        "red" => println!("{}{}", color::Fg(Red), s),
        "blue" => println!("{}{}", color::Fg(Blue), s),
        "green" => println!("{}{}", color::Fg(Green), s),
        "yellow" => println!("{}{}", color::Fg(Yellow), s),
        _ => println!("Invalid color.")
    }
    
}