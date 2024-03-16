use clap::Parser;

use self::fs_util::read_lines_to_vec;
use self::walker::Walker;

mod fs_util;
mod http_resolver;
mod walker;

/// creates cmd line parser impl using clap proc_macros
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

  // reads word list at 'filepath'
  let words = read_lines_to_vec(filepath.into());

  // constructs walker with default num_threads (4)
  let walker = Walker::new(None /* num_threads */, true /* verbose */);
  let resolved_urls = walker.traverse_all(base_path.to_string(), words).unwrap();

  println!("Resolved urls are:");
  for url in resolved_urls {
    println!("\t* {}", url);
  }
}
