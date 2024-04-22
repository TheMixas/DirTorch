use std::path::PathBuf;

pub fn get_file_extension(file_path: &PathBuf, index_from_right: u8) -> Result<String, String> {
    let file_name = file_path.file_name().ok_or("Error while getting file name")?;
    let file_name = file_name.to_str().ok_or("Error while converting file name to string")?;
    let file_name_parts: Vec<&str> = file_name.split('.').collect();
    if file_name_parts.len() < 2 {
        return Err("Error while getting file extension".to_string());
    }
    Ok(file_name_parts[file_name_parts.len() -1 - index_from_right as usize].to_string())
}