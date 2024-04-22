use std::fmt::{Debug, Formatter};
use crate::cli_action_params::CLIActionParams;
use crate::command::Command;

pub struct ChangeDirectoryCommand;

impl Debug for ChangeDirectoryCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ChangeDirectoryCommand")
    }
}

impl Command for ChangeDirectoryCommand {
    fn execute(&self, action_params: CLIActionParams) -> bool {
        change_directory(action_params)
    }
    fn undo(&mut self) {
        println!("Undoing change_directory command");
    }

    fn help(&self) -> &str {
        "cd: changes the current directory"
    }

    fn help_extended(&self) -> &str {
        "cd: changes the current directory
        \n\tUsage: cd <directory>
        \n\tExample: cd /home/user"
    }

    fn should_add_to_history(&self) -> bool {
        false
    }

    fn is_undoable(&self) -> bool {
        false
    }

    fn clone(&self) -> Box<dyn Command + Send + Sync> {
        Box::new(ChangeDirectoryCommand)
    }
}

fn change_directory(p0: CLIActionParams) -> bool {
    let new_dir = match p0.parameters.get(0){
        Some(dir) => dir,
        None => {
            println!("No directory provided!");
            return false;
        }
    };
    let new_dir = std::path::PathBuf::from(new_dir);
    let new_dir = if new_dir.starts_with("/") {
        new_dir
    } else {
        let current_dir = crate::APP_STATE::APP_STATE.get_current_dir();
        let mut new_path = current_dir.clone();
        new_path.push(new_dir);
        new_path
    };
    //Check if the directory exists
    if !new_dir.exists() {
        println!("Directory {} does not exist!", new_dir.display());
        return false;
    }
    crate::APP_STATE::APP_STATE.set_current_dir(new_dir);
    true
}
