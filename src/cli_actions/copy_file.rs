use std::fmt::{Debug, Formatter};
use std::fs::{create_dir_all, File, read_dir};
use std::io::{BufRead, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use crate::APP_STATE::APP_STATE;
use crate::cli_action_params::CLIActionParams;
use crate::command::Command;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::{fs, thread};
use std::thread::JoinHandle;
use rayon::prelude::*;
pub struct CopyFileCommand;

impl Debug for CopyFileCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "CopyFileCommand")
    }
}

impl Command for CopyFileCommand {
    fn execute(&self, action_params: CLIActionParams) -> bool {
        copy_file_multi_threaded_recursive(action_params)
    }
    fn undo(&mut self) {
        println!("Undoing copy_file command");
    }
    fn help(&self) -> &str {
        "copy: copies a file or a dir from one location to another"
    }
    fn help_extended(&self) -> &str {
        "copy: copies a file or a directory from one location to another
        \n\tUsage: copy <source> <destination>
        \n\tExample: copy /home/user/file1.txt /home/user/file2.txt
        \n\tExample: copy /home/user/dir1 /home/user/dir2"
    }

    fn should_add_to_history(&self) -> bool {
        true
    }

    fn is_undoable(&self) -> bool {
        false
    }
//C:\Users\themi\Documents\to_b_cloned2.txt C:\Users\themi\Documents\dir_to_be_cloned
    //copy C:\Users\themi\Documents\dir_to_be_cloned C:\Users\themi\Documents\the_cloned_dir
    fn clone(&self) -> Box<dyn Command + Send + Sync> {
        Box::new(CopyFileCommand)
    }
}
#[deprecated(since = "0.0.1", note = "Please use the `copy_file_multi_threaded_recursive` instead")]

fn copy_file_single_threaded(p0: CLIActionParams) -> bool {

    if p0.parameters.len() < 2 {
        println!("Error while executing copy_file, at least 2 paths are required");
        return false;
    }

    let source_path: PathBuf = match p0.parameters.get(0) {
        Some(value) => {
            println!("Source path: {}", value);
            PathBuf::from(value)
        },
        None => {
            println!("No source provided, not copying file");
            return false;
        }
    };
    let destination_path: PathBuf = match p0.parameters.get(1) {
        Some(value) => {
            println!("Destination path: {}", value);
            PathBuf::from(value)
        },
        None => {
            println!("No destination provided, not copying file");
            return false;
        }
    };
    if(source_path == destination_path){
        println!("Source and destination are the same, not copying file/dir");
        return false;
    }
    if !source_path.exists() {
        println!("Source path does not exist");
        return false;
    }
    if destination_path.exists() {
        println!("Destination path already exists");
        return false;
    }
    if source_path.is_dir() {
        std::fs::create_dir_all(&destination_path).unwrap();
        for entry in std::fs::read_dir(&source_path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let file_name = entry.file_name();
            let mut new_path = destination_path.clone();
            new_path.push(file_name);
            if path.is_dir() {
                std::fs::create_dir_all(&new_path).unwrap();
            } else {
                std::fs::copy(&path, &new_path).unwrap();
            }
        }
    } else {
        std::fs::copy(&source_path, &destination_path).unwrap();
    }
    true
}

fn copy_file_multi_threaded_recursive(p0: CLIActionParams) -> bool {

    if p0.parameters.len() < 2 {
        println!("Error while executing copy_file, at least 2 paths are required");
        return false;
    }

    let source_path: PathBuf = match p0.parameters.get(0) {
        Some(value) => {
            println!("Source path: {}", value);
            PathBuf::from(value)
        },
        None => {
            println!("No source provided, not copying file");
            return false;
        }
    };
    let destination_path: PathBuf = match p0.parameters.get(1) {
        Some(value) => {
            println!("Destination path: {}", value);
            PathBuf::from(value)
        },
        None => {
            println!("No destination provided, not copying file");
            return false;
        }
    };
    if(source_path == destination_path){
        println!("Source and destination are the same, not copying file/dir");
        return false;
    }
    if !source_path.exists() {
        println!("Source path does not exist");
        return false;
    }
    if destination_path.exists() {
        println!("Destination path already exists");
        return false;
    }
    //If target is file, copy it ( single thread )
    if !source_path.is_dir() {
        std::fs::copy(&source_path, &destination_path).unwrap();
        println!("Copied file");
        return true
    }
    copy_directory(&source_path, &destination_path);

    true
}
fn copy_directory(source: &Path, destination: &Path) {
    // Create the destination directory if it doesn't exist
    if !destination.exists() {
        fs::create_dir_all(destination).unwrap();
    }

    // Iterate over entries in the source directory in parallel
    let entries: Vec<_> = fs::read_dir(source)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect();

    // Use Rayon to parallelize the processing of directory entries
    entries.par_iter().for_each(|entry| {
        let destination_file_path = destination.join(entry.file_name().unwrap());
        if entry.is_dir() {
            copy_directory(entry, &destination_file_path);
        } else {
            fs::copy(entry, &destination_file_path).unwrap();
        }
    });
}