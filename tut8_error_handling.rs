use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};
use std::error::Error;
// use std::fs;
// use std::io;

fn main() {

    //--------------explicit errors--------------
    panic!("crash and burn");
    // or
    let v = vec![1, 2, 3];
    v[99];
    
    //--------------result enum--------------
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    // matching different errors
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
    
    //--------------alternative to using result--------------
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    
    //--------------unwrap--------------
    let greeting_file = File::open("hello.txt").unwrap();
    
    //--------------except--------------
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
        
    //--------------propogating error--------------
    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");
        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
        let mut username = String::new();
        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }
    // shortcut
    fn read_username_from_file_2() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }
    // chaining method after calling ?
    fn read_username_from_file_3() -> Result<String, io::Error> {
        let mut username = String::new();
        File::open("hello.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }
    // using fs::read_to_string()
    fn read_username_from_file() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }
    // note
    let greeting_file = File::open("hello.txt")?; // ? operator can not be used in main()
    // changing main return type to result<T, Err>
    // fn main() -> Result<(), Box<dyn Error>> {
    //     let greeting_file = File::open("hello.txt")?;
    //     Ok(())
    // }
    
    //--------------? operator on an option<T>--------------
    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }
    
    //--------------validation loop--------------
    loop {
        // --snip--
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }
        match guess.cmp(&secret_number) {
            // --snip--
    }
    
    //--------------validation function--------------
    pub struct Guess {
        value: i32,
    }
    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }
            Guess { value }
        }
        pub fn value(&self) -> i32 {
            self.value
        }
    }

}


