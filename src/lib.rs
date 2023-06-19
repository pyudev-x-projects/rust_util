/// consts: Constants.
pub mod consts {
    /// DEFAULT_HOMEDIR: If you dont know where you want the homedir for the OS struct to be you can set it to this.
    const DEFAULT_HOMEDIR: &str = "/";
}

/// funcs: Utility functions.
pub mod funcs {
    use std::{
        io::{self, Write, BufReader},
        fs,
        thread::sleep,
        time::Duration,
        process::{self, Output},
    };
    use rodio::{Source, self};
    use termion::color::{self, *};
    
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
    /// 
    /// I dont know the point of this function, its very simple to create without any libraries.
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
        let out = exec("curl", url);
        output(out);
    }

    /// change_color: change the console text color.
    ///
    /// c: The color.
    ///
    /// List of colors: ["red", "blue", "green", "yellow", "magenta", "cyan", "white", "black"].
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

    /// rainbow_hello: pointless function for a rainbow hello world.
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

    /// play_sound: play a sound with the specified path using the rodio library.
    /// 
    /// path: the path for the audio file.
    /// 
    /// duration: the amount of time to sleep before the sound ends.
    pub fn play_sound(path: &str, duration:u64) {
        let stream_handle = rodio::OutputStream::try_default().unwrap();

        let sound_file = BufReader::new(fs::File::open(path).unwrap());

        let source = rodio::Decoder::new(sound_file).unwrap();

        stream_handle.1.play_raw(source.convert_samples()).unwrap();

        sleep(Duration::from_secs(duration));
    }
}

/// jacket: Completely random stuff that are not useful at all but just make programming funny.
/// 
/// Things in this module will not be documented.
pub mod jacket {
    pub struct Human {
        name: String,
        age: u32,
        alive: bool,
    }

    impl Human {
        pub fn new(name: String) -> Human {
            Human { name: name, age: 0, alive: true } 
        }

        pub fn check_alive(self) -> bool {
            return self.alive;
        }

        pub fn kill(&mut self) {
            self.alive = false;
        }

        pub fn talk(self, phrase: &str) {
            if self.alive {
                if self.age >= 3 {
                    println!("{} says: {}", self.name, phrase);
                }
                else {
                    println!("{} must be 3 years or older to talk.", self.name);
                }
            }
            else {
                println!("{} must be alive! Not in a grave!", self.name);
            }
        }
    
        pub fn age(&mut self) {
            self.age += 1;
            println!("Happy birthday {} you are now {} years old", self.name, self.age);
        }
    }

    pub struct Drink {
        name: String,
        mlg: bool
    }

    impl Drink {
        pub fn new(name: &str, mlg: bool) -> Drink {
            return Drink { name: String::from(name), mlg: mlg };
        }

        pub fn drink_it(self) {
            if self.mlg {
                println!("*INTENSE SLURP*, *Drank {}*", self.name);
                println!("Enter the metajuice.");
            }
            else {
                println!("*Slurp*, *Drank {}*", self.name);
            }
        }
    }

    pub enum Bombs {
        Timebomb,
        Dynamite,
        TNT,
        Pipebomb,
        Nuke,
        Japan1945,
        SovietCollapse
    }
}
/// OS: Struct to run OS commands in a specific directory.
/// 
/// homedir: The directory as root directory. Example: you run mkdir("hi"), that would create a directory called hi in the root directory.
pub struct OS {
    homedir:String
}

impl OS {
    /// new: Creates a new OS struct.
    /// 
    /// homedir: The directory as root directory. Example: you run mkdir("hi"), that would create a directory called hi in the root directory. 
    pub fn new(homedir:String) -> OS {
        if homedir.ends_with("/") {
            let os:OS = OS { homedir:  homedir};

            return os;
        }
        else {
            let os:OS = OS { homedir:  homedir+"/"};

            return os;
        }
    }
    /// mkdir: Creates a new directory the 'homedir' property.
    /// 
    /// name: the name of the directory.
    pub fn mkdir(&self, name:&str) {
        let string = self.homedir.to_owned()+name;
        
        funcs::exec("mkdir", &string.trim());
    }

    /// touch: Creates a new file the 'homedir' property.
    /// 
    /// name: the name of the file.
    pub fn touch(&self, name:&str) {
        let string = self.homedir.to_owned()+name;
        
        funcs::exec("touch", &string.trim());
    }

    /// change_dir: Changes the root directory to the specified directory
    /// 
    /// dir: Where the directory is located
    pub fn change_dir(&mut self, dir:String) {
        self.homedir = dir;
    }
}