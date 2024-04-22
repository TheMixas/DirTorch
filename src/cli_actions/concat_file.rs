use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};
use std::path::PathBuf;
use crate::APP_STATE::APP_STATE;
use crate::cli_action_params::CLIActionParams;
use crate::command::Command;
use std::io::Write;

pub struct ConcatFileCommand;

impl Debug for ConcatFileCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConcatFileCommand")
    }
}

impl Command for ConcatFileCommand {
    fn execute(&self, action_params: CLIActionParams) -> bool {
        concat_file(action_params)
    }
    fn undo(&mut self) {
        println!("Undoing concat_file command");
    }
    fn help(&self) -> &str {
        "cat: concatenates the contents of a file and writes it to another file"
    }
    fn help_extended(&self) -> &str {
        "cat: concatenates the contents of a file and writes it to another file
        \n\tUsage: cat <file> <file> -o --<output-file>
        \n\tExample: cat /home/user/file1.txt /home/user/file2.txt -o --/home/user/output.txt"
    }

    fn should_add_to_history(&self) -> bool {
        true
    }

    fn is_undoable(&self) -> bool {
        false
    }

    fn clone(&self) -> Box<dyn Command + Send + Sync> {
        Box::new(ConcatFileCommand)
    }
}
pub fn concat_file(command: CLIActionParams) -> bool{
    if command.parameters.len() < 2 {
        println!("Error while executing concat_file, at least 2 paths are required");
        return false;
    }
    let output_path: PathBuf = match command.flags.get("o") {
        Some(flag_values) => match flag_values.get(0) {
            Some(value) => {
                println!("Output path: {}", value);
                if(value == ""){
                    println!("No output provided, not writing file");
                    return false;
                }
                if(!value.starts_with("/")){
                    let current_dir = APP_STATE.get_current_dir();
                    let mut new_path = current_dir.clone();
                    new_path.push(value);
                    new_path
                }else{
                    PathBuf::from(value)
                }
                // value.clone()
            },
            None => {
                println!("Error while executing concat_file, no output path provided (flag value)");
                return false;
            }
        },
        None => {
            println!("No output path provided");
            return false;
        }
    };
    let current_dir = APP_STATE.get_current_dir();
    let mut output_file = match File::create(&output_path) {
        Ok(file) => BufWriter::new(file),
        Err(e) => {
            println!("Error while creating output file: {}", e);
            return false;
        }
    };


    for path in command.parameters.iter(){
        let absolute_path = if !path.starts_with("/") {
            let mut new_path = current_dir.clone();
            new_path.push(path);
            new_path
        } else {
            PathBuf::from(path)
        };

        // println!("Absolute Path: {:?}", absolute_path);

        let input_file= match File::open(&absolute_path) {
            Ok(file) => BufReader::new(file),
            Err(e) => {
                println!("Error while opening file: {}", e);
                return false;
            }
        };

        for line in input_file.lines() {
            match line {
                Ok(content) => {
                    if let Err(e) = writeln!(output_file, "{}", content) {
                        println!("Error while writing to output file: {}", e);
                        return false;
                    }
                    println!("Writing line: {}", content);
                },
                Err(e) => println!("Error while reading file ({}): {}", absolute_path.display(), e)
            }
        }
    }

    // println!("Output path: {:?}", output_path);
    // println!("Ouutput file: {:?}", output_file);

    println!("Concatenation successful, file written to: {:?}", output_path);
    true
}