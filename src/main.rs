use std::path::PathBuf;

use clap::Parser;

use self::fs_util::read_lines_to_vec;
use self::walker::Walker;

mod fs_util;
mod http_resolver;
mod walker;

#[derive(Parser, Debug)]
struct Config {
  #[clap(short = 'b')]
  base_path: String,
  #[clap(short = 'p')]
  filepath: String,
}

fn main() {
  let Config {
    base_path,
    filepath,
  } = Config::parse();

  let words = read_words_at(filepath.into());

  let walker = Walker::new(None /* num_threads */, true /* verbose */);
  let resolved_urls = walker.traverse_all(base_path.to_string(), words).unwrap();

  println!("Resolved urls are:");
  for url in resolved_urls {
    println!("\t* {}", url);
  }
}

fn read_words_at(path: PathBuf) -> Vec<String> {
  read_lines_to_vec(path)
}
