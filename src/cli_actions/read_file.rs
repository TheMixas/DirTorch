use std::fmt::{Debug, Formatter};
use crate::cli_action_params::CLIActionParams;
use crate::command::Command;
use crate::helper_functions::fix_path::fix_path;

pub struct ReadFileCommand;

impl Debug for ReadFileCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ReadFileCommand")
    }

}

impl Command for ReadFileCommand{
    fn execute(&self, action_params: CLIActionParams) -> bool {
        read_file(action_params)
    }

    fn undo(&mut self) {
        panic!()
    }

    fn help(&self) -> &str {
        "read: prints the text contents of a file
        "
    }

    fn help_extended(&self) -> &str {
        "read: prints the text contents of a file
        \n\tUsage: read <file>
        \n\tExample: read /home/user/file.txt
        "
    }

    fn should_add_to_history(&self) -> bool {
        false
    }

    fn is_undoable(&self) -> bool {
        false
    }

    fn clone(&self) -> Box<dyn Command + Send + Sync> {
        Box::new(ReadFileCommand)
    }
}
pub fn read_file(command: CLIActionParams) -> bool{
    if command.parameters.len() == 0 {
        println!("Error while executing view_file, no path provided");
        return false;
    }
    let path = command.parameters.get(0).unwrap();
    let fixed_path = fix_path(path);
    return match std::fs::read_to_string(fixed_path) {
        Ok(content) => { println!("{}", content);true},
        Err(e) => { println!("Error while reading file: {}", e);false }
    }
}