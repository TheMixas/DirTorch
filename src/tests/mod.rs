use std::{env, fs};
use std::io::{Read, Write};
use std::fs::File;

use std::io::SeekFrom::End;
use std::path::{Path, PathBuf};
use rand::Rng;
use crate::APP_STATE::APP_STATE;
use crate::cli_action_params::CLIActionParams;
use crate::cli_actions::compress::compress;
use crate::cli_actions::decompress::decompress;
use crate::helper_functions::detect_compression_format::detect_compression_format;
use crate::helper_functions::format_time::format_time;
use crate::helper_functions::get_file_extension::get_file_extension;

#[test]
fn print_dynamic_home_dir() {
    println!("dynamic home dir: {:?}", dirs::home_dir().unwrap());
}

#[test]
fn check_compression_format() {
    let file_path = PathBuf::from("C:\\Users\\themi\\Desktop\\rrr.gzip.gzip");
    let result = detect_compression_format(&file_path);
    println!("Compression format: {:?}", result);
}

#[test]
fn check_extension() {
    let file_path = PathBuf::from("C:\\Users\\themi\\Desktop\\rrr.txt.gzip");
    let extension = get_file_extension(&file_path, 1);
    println!("Extension: {:?}", extension);
}
#[test]
fn format_current_time() {
    let time =  std::time::SystemTime::now();
    let formatted = format_time(&time);
    println!("Formatted time: {:?}", formatted);
}
#[test]
fn test_compress_decompress() {
    //SETUP command params
    let file_name = "test.txt".to_string();
    let mut compress_params = CLIActionParams{
        action_name: "compress".to_string(),
        parameters: vec![file_name.clone()],
        flags: std::collections::HashMap::new()
    };

    let mut decompress_params = CLIActionParams{
        action_name: "decompress".to_string(),
        parameters: vec![format!("{}.gz", file_name.clone())],
        flags: std::collections::HashMap::new()
    };

    //SETUP APP STATE FOR TESTING, set current dir to project root
    crate::APP_STATE::APP_STATE.set_current_dir(PathBuf::from(env::current_dir().unwrap()));
    //CURENT DIR = PROJECT ROOT


    let random_string: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    // Write the string to a file
    let mut file = std::fs::File::create(file_name.clone()).unwrap();
    file.write_all(random_string.as_bytes()).unwrap();

    // Compress the file
    let file_path = PathBuf::from("test.txt");
    let compression_succeeded: bool = compress(compress_params);
    println!("Compression succeeded: {}", compression_succeeded);
    // Decompress the file
    let decompression_succeeded: bool = decompress(decompress_params);
    println!("Decompression succeeded: {}", decompression_succeeded);
    let mut decompressed_file= File::open(file_name).unwrap();
    let mut decompressed_string = String::new();
    decompressed_file.read_to_string(&mut decompressed_string).unwrap();

    //ASSERTIONS
    assert_eq!(random_string, decompressed_string);

    //TODO: REMOVE FILES


}