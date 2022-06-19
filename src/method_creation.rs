use crate::bds_data_formatting::{cast_type, name_for_method, FunctionType};

/// Generates a list of getters and setters without the need for member variables using only reinterpret casts and the class `this` pointer
///
/// # Arguments
///
/// * `fields`: contains the member variable names and types
/// * `offsets`: a list which contains the offset for every member variable
/// * `fn_type`: determines whether to generate getters or setters
///
/// returns: String
///
/// # Examples
///
/// ```
///
/// ```
pub fn create_members(
    fields: &Vec<Vec<&str>>,
    offsets: &Vec<i64>,
    fn_type: FunctionType,
) -> String {
    let mut members: String = String::from("");
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
                    members.push_str(&format!("\tauto get{}() -> {}{{\n\t\treturn {}<{}>(reintepret_cast<VA>(this) + 0x{:X}}}); \n\t}}\n", name_for_method(&var_name), &buf_str, cast_type(&buf_str), &buf_str.trim(), offsets[index]));
                }
                FunctionType::SETTERS => {
                    members.push_str(&format!("\tauto set{}({} param_1) -> void {{\n\t\t{}<{}>(reintepret_cast<VA>(this) + 0x{:X}) = param_1;\n\t}}\n", name_for_method(&var_name), &buf_str, cast_type(&buf_str), &buf_str.trim(), offsets[index]))
                }
            }
        }

        index += 1;
    }

    members
}
