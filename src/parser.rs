use std::collections::HashMap;
use crate::cli_action_params::CLIActionParams;

/// Parses a command string into a ParsedCommand struct.
///
/// # Parameters
/// * `input` - A string representing the command to be parsed. The command string should be formatted as follows:
///   "command -flag1 --flag1value1 --flag1value2 -flag2 --flag2value1 parameter1 parameter2"
///   The command string can contain any number of flags, flag values, and parameters.
///
/// # Returns
/// A Result containing a ParsedCommand struct if the command string was successfully parsed, or a String error message if the parsing failed.
/// The ParsedCommand struct contains the following fields:
/// * `action_name`: A String representing the action name.
/// * `flags`: A HashMap where the keys are Strings representing the flags and the values are Vectors of Strings representing the flag values.
/// * `parameters`: A Vector of Strings representing the parameters.
pub fn parse_action(input: &str) -> Result<CLIActionParams, String>{
    let mut parts = input.split_whitespace();
    let command = match parts.next(){
        Some(command)=>command,
        None=>return Err("No command entered".to_string())
    };
    //both flags and parameters
    let mut elements = parts;
    let mut flags: HashMap<&str,Vec<&str>> = HashMap::new();
    let mut parameters: Vec<&str> = Vec::new();

    let mut current_flag = "";
    for element in elements {
        if element.starts_with("-") && !element.starts_with("--"){
            // Check if the element starts with "-" and is a flag
            // If it is, add it to the flags HashMap and set the current_flag to the element
            //remove the dash from the flag
            // let flag = element.split("-").collect::<Vec<&str>>()[1];
            let flag = &element[1..];
            current_flag = flag;
            flags.insert(current_flag, Vec::new());
        }else if element.starts_with("--") {
            // Check if the element starts with "--" and is a flag value
            // If it is, add it to the flags HashMap and set the current_flag to the element
            // let flag_value = element.split("--").collect::<Vec<&str>>()[1];
            let flag_value = &element[2..];
            // flags.get_mut(current_flag).unwrap().push(flag_value); // may panic
            match flags.get_mut(current_flag) {
                Some(flag) => flag.push(flag_value),
                None => return Err(format!("Error, Entered a Flag Value ({}) that is not attached to a flag!", flag_value))
            }
        }
        else {
            // If the element is not a flag or a flag value, it is a parameter
            // Add it to the parameters vector

            parameters.push(element);
            current_flag = "";
        }


        println!("{}", element);
    }
    let parsed_command = CLIActionParams {
        action_name: command.to_string(),
        flags: flags.iter().map(|(k,v)| (k.to_string(), v.iter().map(|x| x.to_string()).collect())).collect(),
        parameters: parameters.iter().map(|x| x.to_string()).collect(),
    };
    return Ok(parsed_command);
}