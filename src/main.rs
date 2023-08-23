mod create;
mod get_event;
mod get_files;
mod hello;
mod inputs;

fn main() {
    hello::hello();
    println!("\nC - Create notes\nD - Display all notes\nE - Exit");
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
                println!("All notes!");
                for file in &files_with_extensions {
                    let mut parts = file.split('.');
                    if let Some(name) = parts.next() {
                        println!("{}", name);
                    }
                }
            }
            'E' => {
                println!("Good bye!");
                break;
            }
            _ => {
                println!("Please choose a valid action. Input should be in English letters!");
            }
        }
    }
}
