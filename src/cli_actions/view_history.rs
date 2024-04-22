use std::fmt::{Debug, Formatter};
use crate::APP_STATE::{APP_STATE, AppState};
use crate::cli_action_params::CLIActionParams;
use crate::command::Command;

pub struct ViewHistoryCommand;

impl Debug for ViewHistoryCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "CreateDirCommand")
    }
}

impl Command for ViewHistoryCommand {
    fn execute(&self, action_params: CLIActionParams) -> bool {
        view_history(action_params);
        true
    }
    fn undo(&mut self) {
        println!("Undoing view history command");
    }

    fn help(&self) -> &str {
        "his: view undoable history"
    }
    fn help_extended(&self) -> &str {
        "history: prints the undoable command history"
    }

    fn should_add_to_history(&self) -> bool {
        false
    }

    fn is_undoable(&self) -> bool {
        false
    }

    fn clone(&self) -> Box<dyn Command + Send + Sync> {
        Box::new(ViewHistoryCommand)
    }
}

pub fn view_history(action_params: CLIActionParams) {
    println!("--- Command history ---");
    APP_STATE.print_history();
}
