use std::collections::HashMap;
use crate::cli_actions::{create_dir, copy_file, view_file, concat_file, view_history, undo_command, change_directory, list_files, file_weight, create_file};
use crate::cli_action_params::CLIActionParams;
use crate::cli_actions::help_actions::help;
use crate::command::Command;
//OLD METHOD
//TODO: DELETE THIS
pub fn GetCLIActions() ->HashMap<String,Box<dyn Command + Send + Sync>> {
    let mut commands: HashMap<String,Box<dyn Command + Send + Sync>> = HashMap::new();
    commands.insert("help".to_string(), Box::new(help::HelpCommand));
    commands.insert("mkdir".to_string(), Box::new(create_dir::CreateDirCommand));
    commands.insert("his".to_string(), Box::new(view_history::ViewHistoryCommand));
    commands.insert("undo".to_string(), Box::new(undo_command::UndoCommand));
    commands.insert("cd".to_string(), Box::new(change_directory::ChangeDirectoryCommand));
    commands.insert("ls".to_string(), Box::new(list_files::ListFilesCommand));
    commands.insert("weigh".to_string(), Box::new(file_weight::FileWeightCommand));
    // commands.insert("copy".to_string(), copy_file::copy_file);
    // commands.insert("read".to_string(), view_file::view_file);
    commands.insert("concat".to_string(), Box::new(concat_file::ConcatFileCommand));
    commands.insert("touch".to_string(), Box::new(create_file::CreateFileCommand));
    commands
}

