mod create;
mod edit;
mod get_event;
mod get_files;
mod hello;
mod inputs;
mod open;
fn main() {
    hello::hello();
    print!("\nC - Create notes\nD - Display all notes\nR - Read selected notes\nE - Edit file\nX - Exit\n");
    loop {
        let event = get_event::get_event();

        match event {
            'C' => {
                let notes_name = inputs::notes_input();
                let files_with_extensions = get_files::get_files();

                let files = files_with_extensions
                    .iter()
                    .map(|file| {
                        let mut parts = file.split('.');
                        parts.next().unwrap_or("")
                    })
                    .collect::<Vec<_>>();

                let notes_name_str: &str = notes_name.as_str();
                if files.contains(&notes_name_str) {
                    println!(
                        "A note with the same name already exists. Please choose a different name."
                    );
                } else if let Err(err) = create::notes_create(&notes_name) {
                    eprintln!("Error creating notes file: {}\n", err);
                } else {
                    println!("Notes file created successfully!\n");
                }
            }
            'D' => {
                let files_with_extensions = get_files::get_files();
                println!("\nAll notes!");
                for file in &files_with_extensions {
                    let mut parts = file.split('.');
                    if let Some(name) = parts.next() {
                        println!("{}", name);
                    }
                }
            }
            'R' => {
                let notes_name = inputs::notes_input();
                let _ = open::notes_read(&notes_name);
            }
            'E' => {
                let notes_name = inputs::notes_input();
                let _ = edit::edit_files(&notes_name);
            }
            'X' => {
                println!("Good bye!\n");
                break;
            }
            _ => {
                println!("Please choose a valid action. Input should be in English letters!");
            }
        }
    }
}
