use std::fmt::{Debug, Formatter};
use crate::APP_STATE::APP_STATE;
use crate::cli_action_params::CLIActionParams;
use crate::cli_actions::create_file::CreateFileCommand;
use crate::command::Command;

pub struct ExitCommand;

impl Debug for ExitCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ExitCommand")
    }

}

impl Command for ExitCommand{
    fn execute(&self, action_params: CLIActionParams) -> bool {
        exit_app();
        true
    }

    fn undo(&mut self) {
        println!("Undoing exit command")
    }

    fn help(&self) -> &str {
        "exit: exits the application"
    }

    fn help_extended(&self) -> &str {
        "exit: exits the application
        \n\t It dies.
        "

    }

    fn should_add_to_history(&self) -> bool {
        false
    }

    fn is_undoable(&self) -> bool {
       false
    }

    fn clone(&self) -> Box<dyn Command + Send + Sync> {
        Box::new(ExitCommand)
    }
}

fn exit_app() {
    std::process::exit(0);
}