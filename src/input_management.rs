use crate::{bds_data_formatting::get_offset, BEClass};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub struct InputFile {
    pub dec_n: usize,
    pub mem_n: usize,
    pub end_n: usize,
}

/// Generates member name and offset lists, delimits the input file sections
///
/// # Arguments
///
/// * `file`: class.txt handle
/// * `file_structure`: reference to a class which will hold the position of the lines that delimit sections
/// * `temp_class`: temporary class which will hold temporary and new data being formatted from the txt file
///
/// returns: ()
///
/// # Examples
///
/// ```
///
/// ```
pub fn loop_through_file(file: &File, file_structure: &mut InputFile, temp_class: &mut BEClass) {
    let reader = BufReader::new(file);
    let mut reset: bool = false;

    for (mut index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        if line.contains("DECLA") {
            file_structure.dec_n = index;
        } else if line.contains("INMEM") {
            file_structure.mem_n = index;
        } else if line.contains("ENDSF") {
            file_structure.end_n = index;
        }

        if line.contains("struct __cppobj") {
            let buf_str = String::from(&line);
            temp_class.beginning = buf_str
                .replace("struct __cppobj", "class")
                .replace("__declspec(align(8))", "")
                .replace("__declspec(align(4))", "")
                .replace("  ", " ");
            temp_class.beginning.push_str("\n{\npublic:\n");
        }

        if file_structure.dec_n != usize::MAX
            && file_structure.mem_n != usize::MAX
            && reset == false
        {
            index = 0;
            reset = true;
            continue;
        }

        let line_contents: Vec<&str> = line.split_whitespace().collect();

        if index > file_structure.dec_n
            && index < file_structure.mem_n
            && !(line.contains("{")
                || line.contains("};")
                || line.contains("/*VFT*/")
                || line.contains("struct __cppobj"))
        {
            temp_class.members.push_str(&format!("{}\n", line));
            temp_class.data.var_names.push(
                String::from(line_contents[line_contents.len() - 1])
                    .replace("*", "")
                    .replace(";", ""),
            )
        } else if index > file_structure.mem_n {
            let actual_line = &line as &str;

            match get_offset(actual_line, &line_contents, &temp_class.data.var_names) {
                Ok(offset) => temp_class
                    .data
                    .offsets
                    .push(i64::from_str_radix(offset, 16).unwrap()),
                Err(_) => continue,
            }
        }
    }
}
