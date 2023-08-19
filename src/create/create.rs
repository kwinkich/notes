use std::fs::File;
use std::io::Result;

pub fn notes_create(file_name: &str) -> Result<()> {
    File::create(format!("{}.txt", file_name))?;
    Ok(())
}
