use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)

    // Can be done with: fs::read_to_string("hello.txt")
}

fn main(){
    
   let result = match read_username_from_file() {
       Ok(name) => name,
       Err(e) => panic!("{:?}", e),
   };
   
   println!("{}", result); 
}
