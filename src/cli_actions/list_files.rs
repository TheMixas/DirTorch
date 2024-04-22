use std::fmt::{Debug, Formatter};
use std::path::PathBuf;
use crate::APP_STATE::APP_STATE;
use crate::cli_action_params::CLIActionParams;
use crate::command::Command;

pub struct ListFilesCommand;

impl Debug for ListFilesCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ListFilesCommand")
    }
}

impl Command for ListFilesCommand {
    fn execute(&self, action_params: CLIActionParams) -> bool {
        list_files(action_params)
    }
    fn undo(&mut self) {
        println!("Undoing list_files command");
    }

    fn help(&self) -> &str {
        "ls: lists the files in the current directory"
    }
    fn help_extended(&self) -> &str {
        "ls: lists the files in the current directory
        \n\tUsage: ls <directory>
        \n\tExample: ls /home/user"
    }

    fn should_add_to_history(&self) -> bool {
        false
    }

    fn is_undoable(&self) -> bool {
        false
    }

    fn clone(&self) -> Box<dyn Command + Send + Sync> {
        Box::new(ListFilesCommand)
    }
}

fn list_files(p0: CLIActionParams) -> bool {
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
    if !path.exists() {
        println!("Directory {} does not exist!", path.display());
        return false;
    }
    let entries = match std::fs::read_dir(path) {
        Ok(entries) => entries,
        Err(_) => {
            println!("Error reading directory");
            return false;
        }
    };
    for entry in entries {
        match entry {
            Ok(entry) => {
                //TODO: distinguish between files and directories
                println!("{}", entry.path().display());
            },
            Err(_) => {
                println!("Error reading entry");
            }
        }
    }
    true
}
