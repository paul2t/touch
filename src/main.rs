use std::{
    env::{self},
    fs::File,
    time::SystemTime,
};

fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>();

    for path in args.iter() {
        if let Ok(file) = File::options()
            .create(true)
            .truncate(false)
            .write(true)
            .open(path)
        {
            if file.set_modified(SystemTime::now()).is_err() {
                println!("Failed to update modified time for {}", path);
            }
        } else {
            println!("Failed to open {}", path);
        }
    }
}
