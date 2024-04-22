use std::fmt::{Debug, Formatter};
use crate::cli_action_params::CLIActionParams;
use crate::command::Command;

pub struct CreateDirCommand;



impl Debug for CreateDirCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "CreateDirCommand")
    }
}

impl Command for CreateDirCommand {
    fn execute(&self, action_params: CLIActionParams) -> bool {
        let result = create_dir(action_params);
        match result {
            Ok(_) => true,
            Err(_) => false
        }
    }
    fn undo(&mut self) {

    }

    fn help(&self) -> &str {
        "mkdir: creates a directory"
    }

    fn help_extended(&self) -> &str {
        "mkdir: creates a directory
        \n\tUsage: mkdir <directory>
        \n\tExample: mkdir /home/user/new_dir"
    }

    fn should_add_to_history(&self) -> bool {
        true
    }

    fn is_undoable(&self) -> bool {
        false
    }

    fn clone(&self) -> Box<dyn Command + Send + Sync> {
        Box::new(CreateDirCommand)
    }
}
// MKDIR {PATH}
pub fn create_dir(command: CLIActionParams)-> Result<(),String>{
    //TODO: make this check fancier
    if command.parameters.len() == 0 {
        println!("Error while executing creating_dir, no path provided");
        return Err("No path provided".to_string());
    }
    println!("creating dir of name: {:?}", command.parameters.get(0).unwrap());
    let path = command.parameters.get(0).unwrap();
    let path = std::path::PathBuf::from(path);
    let path = if path.starts_with("/") {
        path
    } else {
        let current_dir = crate::APP_STATE::APP_STATE.get_current_dir();
        let mut new_path = current_dir.clone();
        new_path.push(path);
        new_path
    };
    //detect error and handle it
    return match std::fs::create_dir(path) {
        Ok(_) => {
            println!("Directory created successfully");
            Ok(())
        },
        Err(e) => {
            println!("Error creating directory: {:?}", e);
            Err(e.to_string())
        }
    }
}


pub const MKDIR_HELP_TEXT: &str = "mkdir: creates a directory
    \n\tUsage: mkdir {PATH}
    \n\tExample: mkdir /home/user/new_dir
    \n\tThis will create a new directory called new_dir in the /home/user directory.";