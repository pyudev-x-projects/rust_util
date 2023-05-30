use std::{io, process};

pub fn input(s: &str)-> String {
    let mut val = String::new();

    eprint!("{}", s);
    io::stdin().read_line(&mut val).expect("Could not read input.");

    return val;
}

pub fn exec(cmd: &str, arg: &str) -> process::Output {
    let output = process::Command::new(cmd).arg(arg).output().expect("Could not run system command");

    return output;
}



// #[cfg(test)]
// mod tests {
//     use super::*;
// }
