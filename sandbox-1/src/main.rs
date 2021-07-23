use std::fs::File;
use std::io;
//use std::io::ErrorKind;
use std::io::Read;



fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    //let f = File::open("hello.txt").unwrap_or_else(|error| {
    //    if error.kind() == ErrorKind::NotFound {
    //        File::create("hello.txt").unwrap_or_else(|error| {
    //            panic!("Problem creating the file: {:?}", error);
    //        })
    //    } else {
    //        panic!("Problem opening the file: {:?}", error);
    //    }
    //});
    read_username_from_file().expect("I expected this!"); 
}
