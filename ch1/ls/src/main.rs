use std::fs;
fn main() {
    println!(".");
    println!("..");
    if let Ok(paths) = fs::read_dir(".") {
        for entry in paths {
            if let Ok(entry) = entry {
                println!("{}", entry.file_name().into_string().unwrap());
            }
        }
    }
}
