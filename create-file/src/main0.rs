use std::fs::File;
use std::io::ErrorKind;

fn main() {

    let f = File::open("hello.txt");
    
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file_created) => file_created,
                Err(error2) => panic!("Problem creating the file: {:?}", error2),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

}
