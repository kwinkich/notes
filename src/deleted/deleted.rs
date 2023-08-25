use std::io::Result;

pub fn deleted(file_name: &str) -> Result<()> {
    std::fs::remove_file(format!("{}.txt", file_name)).expect("Error deleting file");
    println!("File deleted successfully");
    Ok(())
}
