use std::fmt::{Debug, Formatter};
use std::path::PathBuf;
use crate::APP_STATE::APP_STATE;
use crate::cli_action_params::CLIActionParams;
use crate::command::Command;
use walkdir::WalkDir;


pub struct FileWeightCommand;

impl Debug for FileWeightCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "FileWeightCommand")
    }
}

impl Command for FileWeightCommand {
    fn execute(&self, action_params: CLIActionParams) -> bool {
        file_weight(action_params)
    }
    fn undo(&mut self) {
        println!("Undoing file_weight command");
    }

    fn help(&self) -> &str {
        "weigh: prints the weight of a file"
    }
    fn help_extended(&self) -> &str {
        "weigh: prints the weight of a file
        \n\tUsage: file_weight <file>
        \n\tExample: file_weight /home/user/file.txt"
    }

    fn should_add_to_history(&self) -> bool {
        false
    }

    fn is_undoable(&self) -> bool {
        false
    }

    fn clone(&self) -> Box<dyn Command + Send + Sync> {
        Box::new(FileWeightCommand)
    }
}

fn file_weight(p0: CLIActionParams) -> bool {

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
        let file_type = if path.is_file() {
            "file"
        } else {
            "directory"
        };
        let total_size = if path.is_file() {
            std::fs::metadata(path.clone()).unwrap().len()
        } else {
            WalkDir::new(&path)
                .into_iter()
                .filter_map(|entry| entry.ok())
                .filter(|entry| entry.path().is_file())
                .map(|entry| std::fs::metadata(entry.path()).unwrap().len())
                .sum()
        };
        let mb_size = total_size as f64 / 1024.0 / 1024.0;
        println!("The {} {} is {} mb",file_type,path.display(), mb_size);
    } else {
        println!("File {} does not exist!", path.display());
        return false;
    }
    true
}