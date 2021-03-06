use std::error;
use std::fmt;
use std::io::Write;
use std::process;
use std::string;

#[derive(Debug)]
pub struct ClipboardError(pub String);

impl fmt::Display for ClipboardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl error::Error for ClipboardError {}

pub fn write(s: string::String) -> Result<(), Box<dyn error::Error>> {
    let child = process::Command::new("xclip")
        .arg("-in")
        .arg("-selection")
        .arg("clipboard")
        .stdin(process::Stdio::piped())
        .spawn()?;
    let mut stdin = match child.stdin {
        Some(stdin) => stdin,
        None => {
            return Result::Err(Box::new(ClipboardError(
                "Cant open xclip command Stdin".to_string(),
            )));
        }
    };
    stdin.write(s.as_bytes())?;
    Ok(())
}
