pub fn check_message_prefix(msg: &str) -> Result<String, String> {
    let message = msg.to_string();
    for prefix in &crate::config::VALID_PREFIXES {
        if message.starts_with(prefix) {
            return Ok(message[prefix.len()..].to_string());
        }
    }
    Err("Invalid prefix".to_string())
}
