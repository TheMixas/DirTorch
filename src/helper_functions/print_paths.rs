use std::path::PathBuf;

pub fn print_paths(paths: Vec<PathBuf>) {
    for path in paths {
        println!("{}", path.display());
    }
}