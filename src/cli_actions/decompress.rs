use std::fmt::{Debug, format, Formatter};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use flate2::read::{GzEncoder, ZlibDecoder, ZlibEncoder};
use flate2::Compression;
use flate2::read::GzDecoder;
use crate::cli_action_params::CLIActionParams;
use crate::command::Command;
use crate::helper_functions::fix_path::fix_path;
use std::io::prelude::*;
use crate::enums::compression_format::CompressionFormat;
use crate::helper_functions::detect_compression_format::detect_compression_format;
use crate::helper_functions::get_file_extension::get_file_extension;

pub struct DecompressCommand;

impl Debug for DecompressCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "DecompressCommand")
    }
}

impl Command for DecompressCommand {
    fn execute(&self, action_params: CLIActionParams) -> bool {
        decompress(action_params)
    }
    fn undo(&mut self) {
        println!("Undoing decompress command");
    }
    fn help(&self) -> &str {
        "decompress: decompresses a file or directory"
    }
    fn help_extended(&self) -> &str {
        "decompress: decompresses a file or directory
        \n\tUsage: decompress <file> <file> -t --<compression-type>
        \n\tExample: decompress /home/user/file1.txt /home/user/file2.txt -t --gzip\
        \n\tThis command will decompress the files in the given paths.\
        \n\tSupported compression types: gzip, zlib. Default: gzip.\
        "

    }

    fn should_add_to_history(&self) -> bool {
        true
    }

    fn is_undoable(&self) -> bool {
        false
    }

    fn clone(&self) -> Box<dyn Command + Send + Sync> {
        Box::new(DecompressCommand)
    }
}


pub fn decompress(p0: CLIActionParams) -> bool {
        if !check_parameters(&p0) {
            return false;
        }

        let mut success = true;
    //TODO: dont fail the operation if one file fails to decompress, return a list of failed files
        for path in &p0.parameters {
            let mut path = fix_path(path);
            let compression_type: CompressionFormat = match detect_compression_format(&path){
                Ok(compression_type) => compression_type,
                Err(e) => {
                    println!("Error while detecting compression format: {:?}", e);
                    return false;
                }
            };
            //exmaple file name: file.txt.gz

            println!("Decompressing file: {:?}", &path);
            let file = File::open(&path).unwrap();
            //remove the last extension
            let path_without_last_extension = path.file_stem().unwrap().to_str().unwrap();
            let decompressed_path = PathBuf::from(format!("{}\\{}", path.parent().unwrap().display(), path_without_last_extension));
            println!("Decompressed file path: {:?}", decompressed_path);
            let mut decompressed_file = match File::create(decompressed_path.clone()){
                Ok(file) => file,
                Err(e) => {
                    println!("Error while creating decompressed file: {:?}", e);
                    continue;
                }
            };

            println!("Compression type: {:?}", compression_type);
            if !Path::new(&path).exists() {
                println!("Input file {:?} does not exist", path);
                return false;
            }else { println!("Input file {:?} exists", path); }
            let mut decoder : Box<dyn Read>;
            match compression_type {
                CompressionFormat::GZip => {
                    decoder = Box::new(GzDecoder::new(file));
                }
                CompressionFormat::Deflate => {
                    decoder = Box::new(ZlibDecoder::new(file));
                }
                _ => {
                    println!("Error while decompressing, unknown compression type: {:?}", compression_type);
                    return false;
                }
            }
            //copy the decompressed file to the new file
            std::io::copy(&mut *decoder, &mut decompressed_file).unwrap();
            println!("Decompressed file: {:?}", &decompressed_path);
        }
        success
}

fn check_parameters(p0: &CLIActionParams) -> bool {
    if p0.parameters.len() < 1 {
        println!("Error while executing decompress, at least 1 path is required");
        return false;
    }
    true
}
