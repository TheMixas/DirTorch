use std::fmt::{Debug, Formatter};
use std::fs::read_dir;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicUsize;
use std::time::Instant;
use rayon::prelude::*;
use walkdir::WalkDir;
use crate::APP_STATE;
use crate::cli_action_params::CLIActionParams;
use crate::command::Command;
use ignore::WalkBuilder;
use crate::helper_functions::fix_path::fix_path;


pub struct FileSearchCommand;
struct ParsedFlags{
    directory: String,
    single_file: bool,

}
impl Debug for FileSearchCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "FileSearchCommand")
    }
}

impl Command for FileSearchCommand {
    fn execute(&self, action_params: CLIActionParams) -> bool {
        search_file_threaded(action_params)
    }
    fn undo(&mut self) {
        println!("Undoing search_file command");
    }

    fn help(&self) -> &str {
        "search: searches for a file in the current directory"
    }

    fn help_extended(&self) -> &str {
        "search: searches for file/s in the current directory
        \n\tUsage: search <file_name> -d <directory> -m // -m for multiple files
        \n\tExample multiple file search: search Idle.png -d C:\\Users\\themi\\Downloads\\ -m
        \n\tExample single file search: search Idle.png -d C:\\Users\\themi\\Downloads\\"
    }

    fn should_add_to_history(&self) -> bool {
        false
    }

    fn is_undoable(&self) -> bool {
        false
    }

    fn clone(&self) -> Box<dyn Command + Send + Sync> {
        Box::new(FileSearchCommand)
    }
}

/*
fn search_file(p0: CLIActionParams) -> bool {
    let file_name = match p0.parameters.get(0){
        Some(file) => file,
        None => {
            println!("No file name provided!");
            return false;
        }
    };

    let dir: PathBuf = match p0.parameters.get(1){
        Some(dir) => PathBuf::from(dir),
        None => {
            println!("No directory provided, using current!");
            crate::APP_STATE::APP_STATE.get_current_dir()
        }
    };

    let mut found = false;
    let mut found_path = PathBuf::new();
    if dir.is_file() {
        match dir.file_name(){
            Some(name) => {
                if let Some(name_str) = name.to_str() {
                    if name_str == file_name {
                        println!("File found: {:?}", dir);
                        found = true;
                        found_path = dir;
                    }
                }
            },
            None => {}
        }
    }else {
        (found ,found_path) = search_in_dir(dir, file_name);
    }
    if !found {
        println!("File not found!");
    }else{
        println!("File found: {:?}", found_path)
    }
    true
}
*/
fn check_parameters(p0: &CLIActionParams) -> bool {
    if p0.parameters.len() < 1 {
        println!("Error while executing checksum, at least 1 path is required");
        return false;
    }
    true
}
fn parse_flags(p0: &CLIActionParams) -> ParsedFlags {
    let mut directory = "".to_string();
    let mut single_file = true;
    for (key, value) in p0.flags.iter() {
        match key.as_str() {
            "d" => {
                directory = value[0].clone();
            },
            "m" => {
                single_file = false;
            },
            _ => {}
        }
    }
    ParsedFlags{
        directory,
        single_file
    }
}
fn search_file_threaded(p0: CLIActionParams) -> bool {
    if !check_parameters(&p0) {
        return false;
    }

    let parsed_flags = parse_flags(&p0);
    let file_name = p0.parameters.get(0).unwrap();
    let dir = parsed_flags.directory.as_str();
    let dir = fix_path(dir);
    return if(parsed_flags.single_file == true){
        // println!("searching single file");
        search_single_file_in_dir_threaded(&dir, file_name)
    }else{
        println!("searching multiple files");
        search_multiple_files_in_dir_threaded(&dir, file_name)
    }
}

/*
fn search_in_dir(dir:PathBuf, file_name:&str) -> (bool, PathBuf){
    let mut found = false;
    let mut found_path = PathBuf::new();
    for entry in std::fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            if path.file_name().unwrap() == file_name {
                println!("File found: {:?}", path);
                found = true;
                found_path = path;
                break;
            }
        }else {
            (found, found_path) = search_in_dir(path, file_name);
            if(found){
                break;
            }
        }
    }
    (found, found_path)

}

 */
//TODO: im sure thres a more effective way for this.
//TODO: encapsulate repeating code
fn search_single_file_in_dir_threaded(dir:&Path, file_name:&str) -> bool{
    //Step 1 get ALL   subdirectories of adir
    let dirs = Arc::new(Mutex::new(Vec::new()));
    let dir_collection_time = Instant::now(); // Start timer
    let walker = WalkBuilder::new(dir).build_parallel();
    let dirs_clone = Arc::clone(&dirs);
    walker.run(|| {
        let dirs = Arc::clone(&dirs_clone);
        Box::new(move |result| {
            match result {
                Ok(entry) => {
                    if entry.file_type().unwrap().is_dir() {
                        dirs.lock().unwrap().push(entry.into_path());
                    }
                },
                Err(_) => {}
            }
            ignore::WalkState::Continue
        })
    });


    let dir_search_elapsed = dir_collection_time.elapsed();
    println!("Directory collection took: {:?}", dir_search_elapsed);
    println!("Found {} sub-directories", dirs.lock().unwrap().len());
    println!("Searching for file: {} in {} sub-directories", file_name, dirs.lock().unwrap().len());
    let file_search_start_time = Instant::now(); // Start timer

    let total = dirs.lock().unwrap().len();
    let counter = Arc::new(AtomicUsize::new(0));

    //Step 2 search for the file in each directory
    let dirs_clone = dirs.lock().unwrap().clone();
    let found = dirs_clone.into_par_iter().find_any(|dir| {
        let mut found = false;
        let dirEntry = match read_dir(dir) {
            Ok(entry) => entry,
            Err(_) => {
                return false;
            }
        };
        for entry in dirEntry {
            let entry = entry.unwrap();
            let path = entry.path();
                if path.file_name().unwrap() == file_name {
                    found = true;
                    break;
                }

        }
        let count = counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
        let percentage = (count as f32 / total as f32) * 100.0;
        print!("\r{}% done, {} out of {}", percentage, count, total);
        std::io::stdout().flush().unwrap();
        // println!("{} out of {}", count, total);


        found
    });
    let elapsed = file_search_start_time.elapsed();
    println!("File search took: {:?}", elapsed);
    println!("Total time: {:?}", dir_search_elapsed + elapsed);

    if(found.is_none()){
        println!("File not found!");
        return false;
    }else{
        let fileType = if found.clone().unwrap().is_dir() {"Directory"} else {"File"};
        let mut path = found.clone().unwrap();
        //add goal to path
        path.push(file_name);
        println!("{} found: {:?} ", fileType,path);
    }
    found.is_some()
    //search Idle.png C:\Users\themi\Downloads\
}
fn search_multiple_files_in_dir_threaded(dir:&Path, file_name:&str) -> bool{
    //Step 1 get ALL   subdirectories of adir
    let dirs = Arc::new(Mutex::new(Vec::new()));
    let dir_collection_time = Instant::now(); // Start timer
    let walker = WalkBuilder::new(dir).build_parallel();
    let dirs_clone = Arc::clone(&dirs);
    walker.run(|| {
        let dirs = Arc::clone(&dirs_clone);
        Box::new(move |result| {
            match result {
                Ok(entry) => {
                    if entry.file_type().unwrap().is_dir() {
                        dirs.lock().unwrap().push(entry.into_path());
                    }
                },
                Err(_) => {}
            }
            ignore::WalkState::Continue
        })
    });


    let dir_search_elapsed = dir_collection_time.elapsed();
    println!("Directory collection took: {:?}", dir_search_elapsed);
    println!("Found {} sub-directories", dirs.lock().unwrap().len());
    println!("Searching for file: {} in {} sub-directories", file_name, dirs.lock().unwrap().len());
    let file_search_start_time = Instant::now(); // Start timer

    let total = dirs.lock().unwrap().len();
    let counter = Arc::new(AtomicUsize::new(0));

    //Step 2 search for the file in each directory
    let dirs_clone = dirs.lock().unwrap().clone();
    let found_files: Vec<PathBuf> = dirs_clone.into_par_iter().filter_map(|dir| {
        let dirEntry = match read_dir(&dir) {
            Ok(entry) => entry,
            Err(_) => {
                return None;
            }
        };
        for entry in dirEntry {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.file_name().unwrap() == file_name {
                return Some(path);
            }

        }
        let count = counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
        let percentage = (count as f32 / total as f32) * 100.0;
        print!("\r{}% done, {} out of {}", percentage, count, total);
        std::io::stdout().flush().unwrap();
        // println!("{} out of {}", count, total);


        None
    }).collect();
    let elapsed = file_search_start_time.elapsed();
    println!("File search took: {:?}", elapsed);
    println!("Total time: {:?}", dir_search_elapsed + elapsed);

    return if (found_files.len() > 0) {
        println!("Here are the files found:");
        for file_path in found_files {
            println!("{:?}", file_path);
        }
        true
    } else {
        println!("Files not found.");
        false
        // let fileType = if found.clone().unwrap().is_dir() {"Directory"} else {"File"};
        // let mut path = found.clone().unwrap();
        // //add goal to path
        // path.push(file_name);
        // println!("{} found: {:?} ", fileType,path);
    }
    //search Idle.png C:\Users\themi\Downloads\
}