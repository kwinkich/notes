mod create;
mod get_files;
mod inputs;

fn main() {
    loop {
        let notes_name = inputs::notes_input();

        if let Err(err) = create::notes_create(&notes_name) {
            eprintln!("Error creating notes file: {}\n", err);
        } else {
            println!("Notes file created successfully!\n");
        }

        let files = get_files::get_files();
        println!("All notes!");
        for file_name in files {
            println!("{}", file_name);
        }
    }
}
