use std::fs;
use std::path::Path;

pub fn get_files() -> Vec<String> {
    let project_root = Path::new(".")
        .canonicalize()
        .expect("Failed to get project root");

    let mut file_names = Vec::new();

    if let Ok(entries) = fs::read_dir(project_root) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file()
                    && path.extension().is_some()
                    && path.extension().unwrap() == "txt"
                {
                    if let Some(file_name) = path.file_name() {
                        file_names.push(file_name.to_string_lossy().into_owned());
                    }
                }
            }
        }
    }

    file_names
}
