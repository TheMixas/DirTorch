use std::path::PathBuf;
use crate::APP_STATE::APP_STATE;
/// This function is used to fix the path provided as input.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to be fixed.
///
/// # Returns
///
/// * `PathBuf` - A PathBuf instance that represents the fixed path.
///
/// # Behavior
///
/// If the path does not start with "/", it is considered as a relative path.
/// In this case, the function retrieves the current directory from the global APP_STATE,
/// clones it into a new PathBuf instance, and pushes the input path onto it.
///
/// If the path starts with "/", it is considered as an absolute path.
/// In this case, the function simply converts the input path into a PathBuf instance and returns it.
pub fn fix_path(path: &str) -> PathBuf {
    if !path.starts_with("/") {
        let current_dir = APP_STATE.get_current_dir();
        let mut new_path = current_dir.clone();
        new_path.push(path);
        new_path
    } else {
        PathBuf::from(path)
    }
}