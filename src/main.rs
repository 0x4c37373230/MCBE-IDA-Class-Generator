extern crate core;

mod data_parsing;
mod input_file;
mod be_class;

use std::fs::File;
use crate::{be_class::BEClass, input_file::{InputFile, loop_through_file}};

fn main() {
    let file = File::open("./class.txt").unwrap();
    let mut file_structure = InputFile::new();
    let mut out_class = BEClass::new();

    loop_through_file(&file, &mut file_structure, &mut out_class);

    match out_class.setup(&file_structure) {
        Ok(_) => println!("{}", out_class),
        Err(_) => println!("Incorrect, incomplete, or invalid file format")
    }
}
