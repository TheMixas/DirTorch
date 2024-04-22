use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::io::{BufRead, BufReader, BufWriter, Write};
use crate::APP_STATE::APP_STATE;
use crate::cli_action_params::CLIActionParams;
use crate::command::Command;
use crate::command_executor;
use crate::command_executor::CommandExecutor;

pub struct HelpCommand;

impl Debug for HelpCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "HelpCommand")
    }
}

impl Command for HelpCommand {
    fn execute(&self, action_params: CLIActionParams) -> bool {
        help(action_params)
    }
    fn undo(&mut self) {
        println!("Undoing help command");
    }

    fn help(&self) -> &str {
        "help: prints the help message"
    }

    fn help_extended(&self) -> &str {
        "help: prints the help message"
    }

    fn should_add_to_history(&self) -> bool {
        false
    }

    fn is_undoable(&self) -> bool {
        false
    }

    fn clone(&self) -> Box<dyn Command + Send + Sync> {
        Box::new(HelpCommand)
    }
}

fn help(p0: CLIActionParams) -> bool {
    println!("--- Help ---");

    if(p0.parameters.len() > 0){
        let command = match p0.parameters.get(0){
            Some(command) => command,
            None => {
                println!("Command not found");
                return false;
            }
        };
        let commands : &HashMap<String,Box<dyn Command + Send + Sync>> = command_executor::EXECUTOR.get_commands();
        match commands.get(command){
            Some(command) => {
                println!("{}", command.help_extended());
            },
            None => {
                println!("Command not found");
            }
        }
        return true;
    }
    let commands : &HashMap<String,Box<dyn Command + Send + Sync>> = command_executor::EXECUTOR.get_commands();
    let mut writer = BufWriter::new(std::io::stdout());
    for (key, value) in commands.iter() {
        writer.write_all(format!("{}\n", value.help()).as_bytes()).unwrap();
    }
    writer.flush().unwrap();
    true
}
//todo: implement help_extended
