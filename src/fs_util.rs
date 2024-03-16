use std::{fs::read_to_string, path::PathBuf};

/// Reads the content of 'path' split by EOL char
pub fn read_lines_to_vec(path: PathBuf) -> Vec<String> {
  read_to_string(&path)
    .expect(&format!("unable to find {}", path.display()))
    .lines()
    .map(|line| line.to_string())
    .collect()
}
