use std::{
    io::{self, Write, BufReader},
    fs,
    thread::sleep,
    time::Duration,
    process::{self, Output},
};
use rodio::Source;
use rodio;
use termion::color;
use termion::color::*;

/// input: creates a String and then calls io::stdin().read_line to read input for the string.
///
/// s: What to print out before asking for input.
pub fn input(s: &str) -> String {
    let mut val = String::new();

    eprint!("{}", s);
    io::stdin()
        .read_line(&mut val)
        .expect("Could not read input.");

    return val;
}
/// exec: creates a Command with the cmd and arg parameters and returns the output.
///
/// cmd: The command to run.
///
/// arg: The command argument.
pub fn exec(cmd: &str, arg: &str) -> process::Output {
    let output = process::Command::new(cmd)
        .arg(arg)
        .output()
        .expect("Could not run system command");

    return output;
}
/// number_vec: create a vector with a specific amount of numbers.
///
/// n: the amount of numbers for this vector.
pub fn number_vec(n: i32) -> Vec<i32> {
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
pub fn curl(url: &str) {
    let output = exec("curl", url);
    io::stdout().write_all(&output.stdout).unwrap();
}

/// change_color: change the console text color
///
/// c: The color
///
/// List of colors: ["red", "blue", "green", "yellow", "magenta", "cyan", "white", "black"]
pub fn change_color(c: &str) {
    let col = String::from(c);

    match col.to_lowercase().trim() {
        "red" => print!("{}", color::Fg(Red)),
        "blue" => print!("{}", color::Fg(Blue)),
        "green" => print!("{}", color::Fg(Green)),
        "yellow" => print!("{}", color::Fg(Yellow)),
        "magenta" => print!("{}", color::Fg(Magenta)),
        "cyan" => print!("{}", color::Fg(Cyan)),
        "white" => print!("{}", color::Fg(White)),
        "black" => print!("{}", color::Fg(Black)),
        _ => println!("Invalid color."),
    }
}

/// output: print out an Output value
///
/// v: the value to print out
pub fn output(v: Output) {
    io::stdout()
        .write_all(&v.stdout)
        .expect("Could not print to stdout");
}

/// rainbow_hello: pointless function for a rainbow hello world
pub fn rainbow_hello() {
    change_color("red");
    print!("H");
    change_color("yellow");
    print!("e");
    change_color("green");
    print!("l");
    change_color("blue");
    print!("l");
    change_color("magenta");
    print!("o, ");
    change_color("red");
    print!("w");
    change_color("yellow");
    print!("o");
    change_color("green");
    print!("r");
    change_color("blue");
    print!("l");
    change_color("magenta");
    print!("d!");
}

/// play_sound: play a sound with the specified path using the rodio library
/// 
/// path: the path for the audio file
/// 
/// duration: the amount of time to sleep before the sound ends.
pub fn play_sound(path: &str, duration:u64) {
    let stream_handle = rodio::OutputStream::try_default().unwrap();

    let sound_file = BufReader::new(fs::File::open(path).unwrap());

    let source = rodio::Decoder::new(sound_file).unwrap();

    stream_handle.1.play_raw(source.convert_samples()).unwrap();

    sleep(Duration::from_secs(duration));
}
