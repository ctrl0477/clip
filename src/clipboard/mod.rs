use std::io::Write;
use std::process::{Command, Stdio};

/// Copies the given text content to the system clipboard.
///
/// # Arguments
///
/// * `contents` - The text content to copy to the clipboard
///
/// # Errors
///
/// Returns an error if the clipboard cannot be accessed or if the text
/// cannot be copied to the clipboard.
pub fn copy_to_clipboard(contents: &str) -> Result<(), String> {
    let mut child = Command::new("pbcopy")
        .stdin(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Error spawning pbcopy: {}", e))?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(contents.as_bytes())
            .map_err(|e| format!("Error writing to clipboard: {}", e))?;
    }

    let status = child
        .wait()
        .map_err(|e| format!("Error waiting for pbcopy: {}", e))?;

    if status.success() {
        Ok(())
    } else {
        Err("pbcopy command failed".to_string())
    }
}
