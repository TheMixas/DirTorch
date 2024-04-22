use flate2::read::{GzDecoder, DeflateDecoder};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use crate::enums::compression_format::CompressionFormat;

pub fn detect_compression_format(file_path:&PathBuf) -> Result<CompressionFormat, String>{
    println!("detect_compression_format file_path: {:?}", file_path);
    let mut file = match File::open(file_path){
        Ok(file) => file,
        Err(e) => return Err(format!("Error while opening file: {:?}", e))
    };


    //read magic number
    let mut magic_number = [0; 4];
    match file.read_exact(&mut magic_number){
        Ok(_) => (),
        Err(e) => return Err(format!("Error while reading magic number: {:?}", e))
    }
    println!("magic_number: {:?}", magic_number);

    //seek to the beginning of the file
    match file.seek(SeekFrom::Start(0)){
        Ok(_) => (),
        Err(e) => return Err(format!("Error while seeking to the beginning of the file: {:?}", e))
    }

    //match magic number
    match magic_number {
        [0x1F, 0x8B, _, _] => Ok(CompressionFormat::GZip),
        [0x78, 0x01, _, _] | [0x78, 0x9c, _, _] | [0x78, 0x78, _,_]=> Ok(CompressionFormat::Deflate),
        _ => Err("Unknown compression format".to_string())
    }
}
