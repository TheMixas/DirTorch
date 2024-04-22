
use crate::cli_action_params::CLIActionParams;

pub fn view_file(command: CLIActionParams){
    if command.parameters.len() == 0 {
        println!("Error while executing view_file, no path provided");
        return;
    }
    let path = command.parameters.get(0).unwrap();
    match std::fs::read_to_string(path) {
        Ok(content) => println!("{}", content),
        Err(e) => println!("Error while reading file: {}", e)
    }
}