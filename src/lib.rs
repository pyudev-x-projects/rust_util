use std::{io, process};


pub mod funcs {
    use std::{io, process};

    /// input: creates a String and then calls io::stdin().read_line to read input for the string
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
    /// cmd: The command to run
    /// arg: The command argument.
    pub fn exec(cmd: &str, arg: &str) -> process::Output {
        let output = process::Command::new(cmd).arg(arg).output().expect("Could not run system command");

        return output;
    }

    pub fn number_vec(n:i32) -> Vec<i32> {
        let mut vec: Vec<i32> = Vec::new();

        for i in 0..n {
            vec.push(i);
        }

        vec.push(n);

        return vec;
    }
}