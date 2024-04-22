use std::fmt::{Debug, Formatter};
use std::io::stdin;
use crate::APP_STATE::APP_STATE;
use crate::cli_action_params::CLIActionParams;



pub trait Command: Send + Sync + Debug{
    /// Executes the command with the provided parameters.
    ///
    /// This method is part of the `Command` trait and is intended to be overridden by
    /// specific command implementations. It is called when a command needs to be executed.
    ///
    /// # Arguments
    ///
    /// * `action_params` - A struct containing the parameters for the command.
    ///
    /// # Returns
    ///
    /// * `bool` - Returns true if the command execution was successful, false otherwise.
    ///
    /// # Warning
    /// Should not be called directly, use `execute_and_record` instead
    fn execute(&self, action_params: CLIActionParams) -> bool;
    /// Executes the command and records the result if the command execution was successful and should be added to history.
    ///
    /// This method first executes the command by calling the `execute` method with the provided parameters.
    /// The result of the execution is stored in the `succeeded` variable.
    ///
    /// # Arguments
    ///
    /// * `action_params` - A struct containing the parameters for the command.
    ///
    /// # Behavior
    ///
    /// * If the command execution is successful (`succeeded` is true) and the command should be added to history,
    ///   the command is added to the application state history.
    ///
    /// # Example
    ///
    /// ```
    /// let action_params = CLIActionParams::new();
    /// command.execute_and_record(action_params);
    /// ```
    fn execute_and_record(&self, action_params: CLIActionParams){
        let succeeded = self.execute(action_params.clone());
        println!("Succeeded: {}", succeeded);
        if(self.should_add_to_history() && succeeded){
            APP_STATE.add_to_history(self.clone());
        }
    }
    fn undo_carefully(&mut self){
        if self.is_undoable(){
            //Ask for confirmation
            loop {
                println!("Are you sure you want to undo this command? (y/n)");
                print!("> ");
                let mut input = String::new();
                stdin()
                    .read_line(&mut input)
                    .unwrap();
                let input = input.trim();
                if input == "y" {
                    break;
                }
                else if input == "n" {
                    return;
                }
                else {
                    println!("Invalid input, please enter 'y' or 'n'!");
                }
            }
            self.undo();


        }else{
            println!("Command is not undoable");
        }
    }
    /// Undoes the previously executed command.
    ///
    /// This function is part of the `Command` trait and is intended to be overridden by
    /// specific command implementations. It is called when the `undo_carefully` function
    /// determines that it is safe to undo a command.
    ///
    /// # Safety
    ///
    /// This function should not be called directly. Instead, use the `undo_carefully` function,
    /// which checks if the command is undoable before calling this function.
    fn undo(&mut self);
    fn help (&self) -> &str;
    fn help_extended(&self) -> &str;

    fn should_add_to_history(&self) -> bool;
    fn is_undoable(&self) -> bool;
    fn clone(&self) -> Box<dyn Command + Send + Sync>;


}

// impl Debug for CommandStruct {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }
//
// impl Command for CommandStruct{
//     fn execute(&mut self, action_params: CLIActionParams) -> bool {
//         println!("Executing command: {}", self.action_name);
//         true
//     }
//
//     fn undo(&mut self) {
//         println!("Undoing command: {}", self.action_name);
//     }
//
//     fn should_add_to_history(&self) -> bool {
//         true
//     }
//
//     fn is_undoable(&self) -> bool {
//         true
//     }
//
//     fn clone(&self) -> Box<dyn Command + Send + Sync> {
//         Box::new(CommandStruct{
//             action_name: self.action_name.clone(),
//             action_params: self.action_params.clone(),
//         })
//     }
// }