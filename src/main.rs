mod create;
mod deleted;
mod edit;
mod get_event;
mod get_files;
mod get_files_with_ex;
mod hello;
mod inputs;
mod open;
fn main() {
    hello::hello();

    loop {
        println!("\nC - Create notes\nA - Display all notes\nR - Reading selected notes\nE - Edit note\nD - Deleting note\nX - Exit");

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
                    println!("Error creating notes file: {}", err);
                } else {
                    println!("Notes file created successfully!");
                }
            }
            'A' => {
                let files_with_extensions = get_files::get_files();
                println!("\nAll notes!");

                if files_with_extensions.is_empty() {
                    println!("Notes doesn't exist")
                } else {
                    for file in &files_with_extensions {
                        let mut parts = file.split('.');
                        if let Some(name) = parts.next() {
                            println!("{}", name);
                        }
                    }
                }
            }
            'R' => {
                let notes_name = inputs::notes_input();
                let files = get_files_with_ex::get_files_with_ex();

                let notes_name_str: &str = notes_name.as_str();
                if !files.iter().any(|file| file == notes_name_str) {
                    println!("Note doesn't exist");
                } else {
                    let _ = open::notes_read(&notes_name);
                }
            }

            'E' => {
                let notes_name = inputs::notes_input();
                let _ = edit::edit_files(&notes_name);
            }
            'D' => {
                let notes_name = inputs::notes_input();
                let files = get_files_with_ex::get_files_with_ex();

                let notes_name_str: &str = notes_name.as_str();
                if !files.iter().any(|file| file == notes_name_str) {
                    println!("Note doesn't exist");
                } else {
                    let _ = deleted::deleted(&notes_name);
                }
            }
            'X' => {
                println!("Good bye!");
                break;
            }
            _ => {
                println!("Please choose a valid action. Input should be in English letters!");
            }
        }
    }
}
