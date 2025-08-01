#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let mut lmodir = String::new();
    for i in  original.bytes() {
            if i.is_ascii_alphabetic() {
                if i.is_ascii_lowercase() {                    
                    lmodir.push((122 - (i - 97)) as char);
                } else if i.is_ascii_uppercase() {
                    lmodir.push((90 - (i - 65)) as char);
                }
            } else {
                lmodir.push(i as char);
            }
    }
    if lmodir == ciphered {
        return Ok(());
    }
    Err(CipherError {
        expected: lmodir,
    })
    
}
