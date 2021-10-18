use crate::iterator::execute;
mod build_car;
mod update_car;
mod hashmap_car;
mod handle_errors;
mod lifetimes;
mod generic_type;
mod iterator;

use std::fs::File;
use std::io::{Error, Read};
use std::path::PathBuf;
/*
fn read_file_contents(path: PathBuf) -> Result<String, Error> {
    let mut string = String::new();

    // Access a file at a specified path
    // ---------------------------------
    // TODO #1:
    // - Pass variable to `file` variable on success, or
    // - Return from function early if there's an error
    let mut file: File = match File::open(path) {
        Ok(file_handle) => file_handle,
        Err(io_error) => return Err(io_error)
    };

    // Read file contents into `String` variable with `read_to_string`
    // ---------------------------------
    // Success path is already filled in
    // TODO #2: Return from function early if there's an error
    match file.read_to_string(&mut string) {
        Ok(_) => (),
        Err(io_error) => return Err(io_error)
    };

    Ok(string)
}

fn main() {
    if read_file_contents(PathBuf::from("src/main.rs")).is_ok() {
        println!("The program found the main file.");
    }
    if read_file_contents(PathBuf::from("non-existent-file.txt")).is_err() {
        println!("The program reported an error for the file that doesn't exist.");
    }
}
*/
fn main(){
    execute();
}