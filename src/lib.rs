use std::{io, process};

pub mod funcs {
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
}

pub mod types {
    pub struct NumGroup {
        int: isize,
        uint: usize,
        float: f64
    }

    impl NumGroup {
        fn new(int:isize, uint:usize, float:f64) -> NumGroup {
            let val: NumGroup = NumGroup { int: (int), uint: (uint), float: (float) };
            return val;
        }

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
