use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::hash::{DefaultHasher, Hasher};
use std::io::Read;
use crate::cli_action_params::CLIActionParams;
use crate::command::Command;
use crate::helper_functions::fix_path::fix_path;


pub struct GetFileChecksumCommand;

impl Debug for GetFileChecksumCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "GetFileChecksumCommand")
    }
}

impl Command for GetFileChecksumCommand {
    fn execute(&self, action_params: CLIActionParams) -> bool {
        get_file_checksum(action_params)
    }
    fn undo(&mut self) {
        println!("Undoing get_file_checksum command");
    }

    fn help(&self) -> &str {
        "checksum: get file checksum"
    }
    fn help_extended(&self) -> &str {
        "checksum: get file checksum
        \n\tUsage: checksum <file>
        \n\tExample: checksum /home/user/file.txt
        \n\tExample: checksum file.txt"
    }

    fn should_add_to_history(&self) -> bool {
        false
    }

    fn is_undoable(&self) -> bool {
        false
    }

    fn clone(&self) -> Box<dyn Command + Send + Sync> {
        Box::new(GetFileChecksumCommand)
    }


}

fn get_file_checksum(p0: CLIActionParams) -> bool {
    if !check_parameters(&p0) {
        return false;
    }

    let path = p0.parameters.get(0).unwrap();
    let path = fix_path(path);

    let mut file = match File::open(&path){
        Ok(file) => file,
        Err(e) => {
            println!("Error while opening file: {}", e);
            return false;
        }
    };

    let mut hasher= DefaultHasher::new();
    let mut buffer = [0; 1024];
    loop {
        let bytes_read = file.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }
        hasher.write(&buffer[..bytes_read]);
    }

    println!("Checksum for file {:?} is: {}", path, hasher.finish());
    true
}

fn check_parameters(p0: &CLIActionParams) -> bool {
    if p0.parameters.len() < 1 {
        println!("Error while executing checksum, at least 1 path is required");
        return false;
    }
    true
}