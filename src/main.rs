use crate::parser::{parse_action};
use crate::cli_action_params::CLIActionParams;
use crate::command::Command;

mod parser;
mod get_cli_actions;
mod cli_actions;
mod cli_action_params;
mod command;
mod APP_STATE;
mod tests;
mod invoker;
mod command_executor;
mod helper_functions;
mod enums;


fn main() {
    loop {
        display_current_dir();
        print!("> ");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .unwrap();
        let parsed_command  = parse_action(&input);
        safely_execute_command(parsed_command);
        //fixme: print current direction for fun

    }
}

fn safely_execute_command(parsed_command: Result<CLIActionParams,String>) {
    if let Err(e) = parsed_command {
        println!("{}", e);
        return;
    }
    let parsed_command = parsed_command.unwrap();
    command_executor::EXECUTOR.execute_command(parsed_command);

}

fn display_current_dir () {
    let current_dir = APP_STATE::APP_STATE.get_current_dir();
    let canonical_dir = std::fs::canonicalize(current_dir).unwrap();
    println!("Current directory: {}", canonical_dir.display());
}