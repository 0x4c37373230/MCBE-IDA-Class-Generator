use core::fmt;
use crate::{input_file::InputFile, data_parsing::{cast_type, ClassInfo, FunctionType, name_for_method}};

pub struct BEClass {
    pub beginning: String,
    pub members: String,
    getters: String,
    setters: String,
    internal_fns: String,
    end: String,
    pub data: ClassInfo,
}

impl fmt::Display for BEClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}{}", self.beginning, self.getters, self.setters, self.end)
    }
}

impl BEClass {
    pub fn new() -> Self {
        Self {
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
        }
    }

    /// Generates the output class body using previously parsed data
    ///
    /// # Arguments
    ///
    /// * `file_structure`: Contains the position of the lines that delimit the file sections
    ///
    /// returns: Result<(), bool>
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn setup(&mut self, file_structure: &InputFile) -> Result<(), bool> {
        match file_structure.check_integrity() {
            Ok(_) => {
                let members = self.members.clone();
                let member_variables: Vec<&str> = members.split("\n").collect();
                let mut new_methods: Vec<Vec<&str>> = Vec::new();

                for line in member_variables {
                    new_methods.push(line.trim().split(" ").collect())
                }

                self.generate_methods(&new_methods, FunctionType::GETTERS);
                self.generate_methods(&new_methods, FunctionType::SETTERS);

                Ok(())
            }
            Err(_) => {
                println!("Incorrect or incomplete file structure");
                Err(false)
            }
        }
    }

    /// Generates a list of getters and setters without the need for member variables using only reinterpret casts and the class `this` pointer
    ///
    /// # Arguments
    ///
    /// * `fields`: contains the member variable names and types
    /// * `fn_type`: determines whether to generate getters or setters
    ///
    /// returns: String
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn generate_methods(&mut self, fields: &Vec<Vec<&str>>, fn_type: FunctionType) {
        let mut index: usize = 0;

        for str_vec in fields {
            let mut buf_str: String = String::from("");
            let mut var_name: String = String::from("");

            for &str in str_vec {
                if !(str.starts_with("m") || str.starts_with("*m")) {
                    buf_str.push_str(str);
                    buf_str.push_str(" ");
                } else {
                    var_name.push_str(str);

                    if str.contains("*") {
                        buf_str.push_str("*");
                    }
                }
            }
            if &buf_str != " " && &var_name != " " {
                match fn_type {
                    FunctionType::GETTERS => {
                        self.getters.push_str(&format!("\tauto get{}() -> {}{{\n\t\treturn {}<{}>(reintepret_cast<VA>(this) + 0x{:X}); \n\t}}\n", name_for_method(&var_name), &buf_str, cast_type(&buf_str), &buf_str.trim(), self.data.offsets[index]));
                    }
                    FunctionType::SETTERS => {
                        self.setters.push_str(&format!("\tauto set{}({} param_1) -> void {{\n\t\t{}<{}>(reintepret_cast<VA>(this) + 0x{:X}) = param_1;\n\t}}\n", name_for_method(&var_name), &buf_str, cast_type(&buf_str), &buf_str.trim(), self.data.offsets[index]))
                    }
                }
            }
            index += 1;
        }
    }
}