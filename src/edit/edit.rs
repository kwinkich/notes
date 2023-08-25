use std::fs::OpenOptions;
use std::io::{stdin, Read, Result, Write};

pub fn edit_files(file_name: &str) -> Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(format!("{}.txt", file_name))
        .expect("Error opening/creating file!");

    let mut file_data = String::new();
    file.read_to_string(&mut file_data)
        .expect("Error reading file!");

    if file_data.is_empty() {
        println!("File data is empty");
    } else {
        println!("File data:{}", file_data);
    }

    println!("Enter your note: ");
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Error getting user input");

    file.write_all(input.as_bytes())
        .expect("Error writing to file!");

    Ok(())
}
