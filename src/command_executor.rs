use std::collections::HashMap;
use std::fmt::Debug;
use lazy_static::lazy_static;
use crate::cli_action_params::CLIActionParams;
use crate::cli_actions::{change_directory, compress, concat_file, copy_file, create_dir, create_file, decompress, file_weight, get_file_checksum, get_file_metadata, list_files, search_file, undo_command, view_history, exit};
use crate::cli_actions::help_actions::help;
use crate::command;
use crate::command::Command;

pub struct CommandExecutor{
    commands: HashMap<String,Box<dyn Command + Send + Sync>>
}

lazy_static!{
    pub static ref EXECUTOR: CommandExecutor = CommandExecutor::new();
}
impl CommandExecutor {

pub fn new() -> CommandExecutor {
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
    commands.insert("copy".to_string(), Box::new(copy_file::CopyFileCommand));
    commands.insert("search".to_string(), Box::new(search_file::FileSearchCommand));

    commands.insert("compress".to_string(), Box::new(compress::CompressCommand));
    commands.insert("decompress".to_string(), Box::new(decompress::DecompressCommand));

    commands.insert("meta".to_string(), Box::new(get_file_metadata::GetFileMetadataCommand));

    commands.insert("chksum".to_string(), Box::new(get_file_checksum::GetFileChecksumCommand));

    commands.insert("exit".to_string(), Box::new(exit::ExitCommand));
    //Todo:
        CommandExecutor{
            commands
        }
    }

    pub fn get_instance() -> &'static CommandExecutor {
        &EXECUTOR
    }

    pub fn execute_command(&self, action_params: CLIActionParams) -> bool {
        let command = self.commands.get(&action_params.action_name);
        if let None = command {
            println!("Command not found");
            return false;
        }
        let command = command.unwrap();
        command.execute(action_params)
    }

    pub fn get_commands(&self) -> &HashMap<String,Box<dyn Command + Send + Sync>> {
        &self.commands
    }

}
