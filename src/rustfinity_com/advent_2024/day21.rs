// https://www.rustfinity.com/practice/rust/challenges/aor-2024-21/description

use std::fs::{remove_file, File, OpenOptions};
use std::io::{Error, ErrorKind, Read, Write};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct TempFile {
  file_path: PathBuf,
  file: File,
}
impl TempFile {
  pub fn new() -> Result<Self, Error> {
    let file_name = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .map_err(|_| ErrorKind::InvalidData)?
      .as_nanos()
      .to_string();
    let file_path = std::env::temp_dir().join(file_name);
    let file = File::create_new(file_path.clone())?;
    Ok(Self { file_path, file })
  }
  pub fn write(&self, data: &[u8]) -> Result<(), Error> {
    let mut file = OpenOptions::new().write(true).open(&self.file_path)?;
    file.write_all(data)?;
    Ok(())
  }
  pub fn read_to_string(&self) -> Result<String, Error> {
    let mut file = OpenOptions::new().read(true).open(&self.file_path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer)
  }
  pub fn path(&self) -> &PathBuf {
    &self.file_path
  }
  pub fn file(&self) -> &File {
    &self.file
  }
}
impl Drop for TempFile {
  fn drop(&mut self) {
    let _ = remove_file(self.path());
  }
}
