use std::path::Path;
use std::fs::{OpenOptions};
use std::io::Write;
use std::fs::File;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {

    let path_ref = path.as_ref();

    if !path_ref.exists() {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(path_ref)
            .expect("Failed to create file");
        file.write_all(content.as_bytes())
            .expect("Failed to write to file");
    } else {
        File::open(path_ref).unwrap();
    }
}