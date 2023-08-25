use crate::get_files::get_files;
use std::vec::Vec;

pub fn get_files_with_ex() -> Vec<String> {
    let files_with_extensions = get_files::get_files();

    let files: Vec<String> = files_with_extensions
        .iter()
        .map(|file| file.split('.').next().unwrap_or_default().to_string())
        .collect();

    files
}
