use std::fmt::{Debug, Formatter};
use std::path::PathBuf;
use crate::APP_STATE::APP_STATE;
use crate::cli_action_params::CLIActionParams;
use crate::command::Command;

pub struct CreateFileCommand;

impl Debug for CreateFileCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "CreateFileCommand")
    }
}

impl Command for CreateFileCommand {
    fn execute(&self, action_params: CLIActionParams) -> bool {
        create_file(action_params)
    }
    fn undo(&mut self) {
        println!("Undoing create_file command");
    }

    fn help(&self) -> &str {
        "touch: creates a file"
    }
    fn help_extended(&self) -> &str {
        "touch: creates a file
        \n\tUsage: touch <file>
        \n\tExample: touch /home/user/file.txt
        \n\tExample: touch file.txt"
    }

    fn should_add_to_history(&self) -> bool {
        false
    }

    fn is_undoable(&self) -> bool {
        false
    }

    fn clone(&self) -> Box<dyn Command + Send + Sync> {
        Box::new(CreateFileCommand)
    }
}

fn create_file(p0: CLIActionParams) -> bool {

    let path: PathBuf = match p0.parameters.get(0){
        Some(dir) => PathBuf::from(dir),
        None => {
            crate::APP_STATE::APP_STATE.get_current_dir()
        }
    };
    let path = if path.starts_with("/") {
        path
    } else {
        let current_dir = crate::APP_STATE::APP_STATE.get_current_dir();
        let mut new_path = current_dir.clone();
        new_path.push(path);
        new_path
    };
    //Check if the directory exists
    if path.exists() {
        println!("File already exists");
        return false;
    }
    match std::fs::File::create(&path) {
        Ok(_) => {
            println!("File created successfully, at path: {:?}", path);
            true
        },
        Err(e) => {
            println!("Error creating file: {}", e);
            false
        }
    }
}