use std::fs::File;

pub fn open_file(s: &str) -> File {
    File::open(s).unwrap()
}

// $ cargo run
// File { fd: 3, path: ".../src/created.txt", read: true, write: false }

// thread 'main' panicked at src/main.rs:
// called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
// ...