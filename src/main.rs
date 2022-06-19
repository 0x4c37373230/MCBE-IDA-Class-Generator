mod bds_data_formatting;
mod class_setup;
mod input_management;
mod method_creation;

use crate::{
    bds_data_formatting::ClassInfo,
    class_setup::final_class_setup,
    input_management::{loop_through_file, InputFile},
};
use std::fs::File;

pub struct BEClass {
    beginning: String,
    members: String,
    getters: String,
    setters: String,
    internal_fns: String,
    end: String,
    data: ClassInfo,
}

fn main() {
    let file = File::open("./class.txt").unwrap();
    let mut file_structure = InputFile {
        dec_n: usize::MAX,
        mem_n: usize::MAX,
        end_n: usize::MAX,
    };
    let mut temp_class = BEClass {
        beginning: "".to_string(),
        members: "".to_string(),
        getters: "".to_string(),
        setters: "".to_string(),
        internal_fns: "".to_string(),
        end: "};".to_string(),
        data: ClassInfo {
            var_names: Vec::new(),
            offsets: Vec::new(),
        },
    };

    loop_through_file(&file, &mut file_structure, &mut temp_class);

    if file_structure.dec_n == usize::MAX
        || file_structure.mem_n == usize::MAX
        || file_structure.end_n == usize::MAX
    {
        println!("Incorrect or incomplete file format");
        return;
    }

    final_class_setup(&mut temp_class);

    println!(
        "{}{}{}{}",
        temp_class.beginning, temp_class.getters, temp_class.setters, temp_class.end
    );
}
