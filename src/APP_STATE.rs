use std::collections::HashMap;
use std::sync::Mutex;
use std::path::PathBuf;
use std::env;
use std::fmt::Debug;
use lazy_static::lazy_static;
use crate::cli_action_params::CLIActionParams;
use crate::command::Command;

pub struct AppState {
    current_dir: Mutex<PathBuf>,
    // action_history: Vec<Box<dyn Command>>,
    action_history: Mutex<Vec<Box<dyn Command + Send + Sync>>>,
    //all possible actions ( commands )
}

impl AppState {
    pub fn get_current_dir(&self) -> PathBuf {
        let lock = self.current_dir.lock().unwrap();
        lock.clone()
    }

    pub fn set_current_dir(&self, new_dir: PathBuf) {
        let mut lock = self.current_dir.lock().unwrap();
        *lock = new_dir;
    }

    pub fn add_to_history(&self, command: Box<dyn Command + Send + Sync>) {
        if(command.should_add_to_history()){
            let mut lock = self.action_history.lock().unwrap();
            lock.push((command));
            println!("Added command to history")
        }
    }

    pub fn undo_last_command(&self) {
        let mut lock = self.action_history.lock().unwrap();
        if let Some(mut command) = lock.pop() {
            command.undo_carefully();
        }else{
            println!("No commands to undo");
        }
    }

    pub fn print_history(&self) {
        // println!("Printing history");
        let lock = self.action_history.lock().unwrap();
        if(lock.len() == 0){
            println!("No commands in history");
            return;
        }
        for command in lock.iter() {
            println!("{:?}", command);
        }
    }

}

lazy_static! {
    pub static ref APP_STATE: AppState = AppState {
        current_dir: Mutex::new(dirs::home_dir().unwrap()),
        action_history: Mutex::new(Vec::new())
    };
}
