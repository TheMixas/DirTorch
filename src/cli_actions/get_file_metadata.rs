use std::fmt::{Debug, Formatter};
use std::fs::metadata;
use std::time::SystemTime;
use crate::cli_action_params::CLIActionParams;
use crate::command::Command;
use crate::helper_functions::fix_path::fix_path;
use crate::helper_functions::format_time::format_time;
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[cfg(windows)]
use std::os::windows::fs::MetadataExt;
use std::io;

pub struct GetFileMetadataCommand;

impl Debug for GetFileMetadataCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "GetFileMetadataCommand")
    }
}

impl Command for GetFileMetadataCommand {
    fn execute(&self, action_params: CLIActionParams) -> bool {
        get_file_metadata(action_params)
    }
    fn undo(&mut self) {
        println!("Undoing get_file_metadata command");
    }

    fn help(&self) -> &str {
        "ls: list directory contents"
    }
    fn help_extended(&self) -> &str {
        "ls: list directory contents
        \n\tUsage: ls <dir>
        \n\tExample: ls /home/user
        \n\tExample: ls"
    }

    fn should_add_to_history(&self) -> bool {
        false
    }

    fn is_undoable(&self) -> bool {
        false
    }

    fn clone(&self) -> Box<dyn Command + Send + Sync> {
        Box::new(GetFileMetadataCommand)
    }
}

fn get_file_metadata(p0: CLIActionParams) -> bool {
    if !check_parameters(&p0) {
        return false;
    }

    let path = p0.parameters.get(0).unwrap();
    let path = fix_path(&path);
    //check that the path exists
    if !path.exists() {
        println!("Error while executing get_file_metadata, path does not exist: {:?}", path);
        return false;
    }

    let m_data = match metadata(&path) {
        Ok(metadata) => {
            metadata
        },
        Err(e) => {
            println!("Error while executing get_file_metadata: {:?}", e);
            return false;
        }
    };
    let permissions = m_data.permissions();

    let permissions_string : String;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = permissions.mode();
        permissions_string = format_permissions(mode);
    };
    #[cfg(windows)]
    {
        if permissions.readonly() {
          permissions_string="The file is read-only.".to_string()
        } else {
            permissions_string="The file is not read-only.".to_string()
        }
    };

    //Print metada in a human readable format
    println!("Metadata for file: {:?}", &path);
    println!("File type: {:?}", format_file_type(&m_data.file_type()));
    println!("File size as mb:[{}], kb:[{}] , b:[{}]", m_data.len() / 1024 / 1024, m_data.len() / 1024, m_data.len());
    println!("Permissions: {permissions_string}");
    println!("Created at| {}", format_time(&m_data.created().unwrap()));
    println!("Modified at| {}", format_time(&m_data.modified().unwrap()));
    println!("Accessed at| {}", format_time(&m_data.accessed().unwrap()));
    true
}

fn check_parameters(p0: &CLIActionParams) -> bool {
    if p0.parameters.len() < 1 {
        println!("Error while executing get_file_metadata, at least 1 path is required");
        return false;
    }
    true
}
fn format_permissions(mode: u32) -> String {
    let user = ((mode >> 6) & 0x7) as u8;
    let group = ((mode >> 3) & 0x7) as u8;
    let others = (mode & 0x7) as u8;

    format!(
        "{}{}{}",
        format_permission(user),
        format_permission(group),
        format_permission(others)
    )
}

fn format_permission(value: u8) -> String {
    let read = if (value & 0x4) != 0 { "r" } else { "-" };
    let write = if (value & 0x2) != 0 { "w" } else { "-" };
    let execute = if (value & 0x1) != 0 { "x" } else { "-" };

    format!("{}{}{}", read, write, execute)
}

fn format_file_type(file_type: &fs::FileType) -> String {
    if file_type.is_dir() {
        return "Directory".to_string();
    }
    if file_type.is_file() {
        return "File".to_string();
    }
    if file_type.is_symlink() {
        return "Symlink".to_string();
    }
    "Unknown".to_string()
}