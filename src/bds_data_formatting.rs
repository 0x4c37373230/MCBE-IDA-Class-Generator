pub struct ClassInfo {
    pub var_names: Vec<String>,
    pub offsets: Vec<i64>,
}

pub enum FunctionType {
    GETTERS,
    SETTERS,
}

/// parses the member variable name into a string to be used in the name for a getter or setter
///
/// # Arguments
///
/// * `name`: member variable name
///
/// returns: &str
///
/// # Examples
///
/// ```
/// format!("get{}() {{\n\t;\n}}", name_for_method("*mLegacyBlock"));
/// // will return "getLegacyBlock() {\n\t;\n}"
/// ```

pub fn name_for_method(name: &str) -> &str {
    return if name.starts_with("m") {
        &name[1..name.len()]
    } else if name.starts_with("*m") {
        &name[2..name.len()]
    } else {
        &name
    };
}

/// If the getter return type is a pointer to something, the reinterpret cast shouldn't be
/// dereferenced. If it isn't, it should
///
/// # Arguments
///
/// * `buffer`: getter return type
///
/// returns: &str
///
/// # Examples
///
/// ```
/// cast_type("BlockPos *"); // will return "reinterpret_cast"
/// ```
pub fn cast_type(buffer: &str) -> &str {
    return if buffer.contains("*") {
        "reinterpret_cast"
    } else {
        "*reinterpret_cast"
    };
}

/// Gets the offset for a specific member variable from the class memory layout extracted from the
/// IDA 'Structures' section
///
/// # Arguments
///
/// * `line`: line from the input file inside the memory layout section
/// * `line_contents`: contents of the line, to easily be able to get the offset
/// * `member_names`: member variable name list
///
/// returns: Result<&str, i64>
///
/// # Examples
///
/// ```
///
/// ```
pub fn get_offset<'a>(
    line: &str,
    line_contents: &'a Vec<&str>,
    member_names: &Vec<String>,
) -> Result<&'a str, i64> {
    for name in member_names {
        if line.contains(name) {
            return Ok(line_contents[0]);
        }
    }

    Err(-1)
}
