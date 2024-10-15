use std::process::{Command, Output};
use std::str;

pub fn picture_ocr(args: &[&str]) -> Result<String, Box<dyn std::error::Error>> {
    let output: Output = Command::new("D:/Download/tesseract/tesseract.exe")
        .args(args)
        .output()
        .map_err(|e| e.to_string())?;
    if output.status.success() {
        if let Ok(stdout) = str::from_utf8(&output.stdout) {
            Ok(stdout.to_string())
        } else {
            Err("213".into())
        }
    } else {
        let stderr = str::from_utf8(&output.stderr).unwrap_or("Unknown error occurred");
        Err(stderr.into())
    }
}
