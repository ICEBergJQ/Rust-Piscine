#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let mut lmodir = String::new();
    for i in  original.chars() {
            if i.is_ascii_alphabetic() {
                let base = if i.is_lowercase() { 'a' } else { 'A' };
                let base2 = if i.is_lowercase() { 'z' } else { 'Z' };
                let mirrored =(base as u8 + (base2 as u8 - i as u8)) as char;
                lmodir.push(mirrored);
            } else {
                lmodir.push(i);
        }
    }
    if lmodir == ciphered {
        Ok(())
    } else {
        Err(CipherError {
            expected: lmodir,
        })
    }
}


