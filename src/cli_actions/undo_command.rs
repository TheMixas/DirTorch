use std::fmt::{Debug, Formatter};
use crate::APP_STATE::APP_STATE;
use crate::cli_action_params::CLIActionParams;
use crate::command::Command;
pub struct UndoCommand;

impl Debug for UndoCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "UndoCommand")
    }
}

impl Command for UndoCommand {
    fn execute(&self, action_params: CLIActionParams) -> bool {
        undo(action_params);
        true
    }
    fn undo(&mut self) {
        println!("Undoing undo command");
    }

    fn help(&self) -> &str {
        "undo: undoes the last undoable command"
    }
    fn help_extended(&self) -> &str {
        "undo: undoes the last undoable command"
    }

    fn should_add_to_history(&self) -> bool {
        false
    }

    fn is_undoable(&self) -> bool {
        false
    }

    fn clone(&self) -> Box<dyn Command + Send + Sync> {
        Box::new(UndoCommand)
    }

}

fn undo(action_params: CLIActionParams) {
    APP_STATE.undo_last_command();
}