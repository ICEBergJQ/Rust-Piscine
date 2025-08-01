pub fn check_ms(message: &str) -> Result<&str, &str> {
    if message.contains("stupid") {
        Err("Message contains profanity")
    } else {
        Ok("ERROR: illegal")
    }
}