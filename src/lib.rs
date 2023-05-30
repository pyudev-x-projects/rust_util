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
}

pub mod types {
    pub struct NumGroup {
        int: isize,
        uint: usize,
        float: f64
    }

    impl NumGroup {
        /// new: creates a new NumGroup and then returns it with the arguments
        /// int: The integer of the group
        /// uint: The unsigned integer of the group
        /// float: the floating point number or decimal of the group
        fn new(int:isize, uint:usize, float:f64) -> NumGroup {
            let val: NumGroup = NumGroup { int: (int), uint: (uint), float: (float) };
            return val;
        }

        /// as_str: returns the NumGroup as a String
        fn as_str(self) -> String {
            let val = format!("int: {}, uint: {}, float: {}", self.int, self.uint, self.float);

            return val;
        }
    }

}

// #[cfg(test)]
// mod tests {
//     use super::*;
// }
