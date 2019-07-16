use std::{io::Error, io::ErrorKind, fs::File};

fn main() {
    println!("Hello, world!");
    let f: Result<File, Error> = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("problem creating the file {:?}", e),
            },
            other_error => panic!("problem opening the file {:?}", other_error),
        },
    };
}
