use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use flate2::bufread::{GzEncoder, ZlibEncoder};
use flate2::Compression;
use crate::cli_action_params::CLIActionParams;
use crate::command::Command;
use crate::helper_functions::fix_path::fix_path;

pub struct CompressCommand;

impl Debug for CompressCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "CompressCommand")
    }
}

impl Command for CompressCommand {
    fn execute(&self, action_params: CLIActionParams) -> bool {
        compress(action_params)
    }
    fn undo(&mut self) {
        println!("Undoing compress command");
    }
    fn help(&self) -> &str {
        "compress: compresses a file or directory"
    }
    fn help_extended(&self) -> &str {
        "compress: compresses a file or directory
        \n\tUsage: compress <file> <file> -t --<compression-type>
        \n\tExample: compress /home/user/file1.txt /home/user/file2.txt -t --gzip"
    }

    fn should_add_to_history(&self) -> bool {
        true
    }

    fn is_undoable(&self) -> bool {
        false
    }

    fn clone(&self) -> Box<dyn Command + Send + Sync> {
        Box::new(CompressCommand)
    }
}
fn check_parameters(p0: &CLIActionParams) -> bool {
    if p0.parameters.len() < 1 {
        println!("Error while executing compress, at least 1 path is required");
        return false;
    }
    true
}
pub fn compress(p0: CLIActionParams) -> bool {

    if !check_parameters(&p0) {
        return false;
    }
    let compression_type: String = p0.flags.get("t")
        .and_then(|flag_values| flag_values.get(0))
        .map(|value| {
            println!("Compression type: {}", value);
            value.to_string()
        })
        .unwrap_or_else(|| {
            println!("No compression type provided, using GZip as default");
            "gzip".to_string()
        });

    let input_path = &p0.parameters[0];
    let output_path = input_path;
    let output_path = fix_path(output_path);
    let input_path = fix_path(input_path);
    if compression_type == "gzip" {
        return compress_g_zip(&input_path, &output_path);
    } else if compression_type == "zlib" {
        return compress_zlib(&input_path, &output_path);
    }
    else {
        println!("Compression type not supported");
        return false;
    }
    false
}

fn compress_g_zip(input: &PathBuf, output: &PathBuf) -> bool {

    println!("Compressing file {:?} to {:?}", input, output);
    let mut input = BufReader::new(File::open(input).unwrap());
    let output = File::create(format!("{}.gz", output.display())).unwrap();
    let mut gz = GzEncoder::new(input, Compression::default());
    let mut output = BufWriter::new(output);
    std::io::copy(&mut gz, &mut output).unwrap();
    println!("Compressed file: {:?}", output);
    true

}
fn compress_zlib(input: &PathBuf, output: &PathBuf) -> bool {
    println!("Compressing file {:?} to {:?}", input, output);
    let mut input = BufReader::new(File::open(input).unwrap());
    let output = File::create(format!("{}.zz", output.display())).unwrap();
    let mut zlib = ZlibEncoder::new(input, Compression::default());
    let mut output = BufWriter::new(output);
    std::io::copy(&mut zlib, &mut output).unwrap();
    println!("Compressed file: {:?}", output);
    true
}