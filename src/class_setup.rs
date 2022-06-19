use crate::{bds_data_formatting::FunctionType, method_creation::create_members, BEClass};

/// Generates the output class from parsed data
///
/// # Arguments
///
/// * `temp`: temporary new class
///
/// returns: ()
///
/// # Examples
///
/// ```
///
/// ```
pub fn final_class_setup(temp: &mut BEClass) {
    let member_variables: Vec<&str> = temp.members.split("\n").collect();
    let mut new_methods: Vec<Vec<&str>> = Vec::new();

    for line in member_variables {
        new_methods.push(line.trim().split(" ").collect())
    }

    temp.getters = create_members(&new_methods, &temp.data.offsets, FunctionType::GETTERS);
    temp.setters = create_members(&new_methods, &temp.data.offsets, FunctionType::SETTERS);
}
