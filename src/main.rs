use clap::Parser;

use self::fs_util::read_lines_to_vec;

mod fs_util;
mod resolver;

/// creates cmd line parser impl using clap proc_macros
#[derive(Parser, Debug)]
struct Config {
  #[arg(short = 'b')]
  base_path: String,
  #[arg(short = 'p')]
  filepath: String,
  #[arg(long, action)]
  verbose: bool,
}

fn main() {
  let Config {
    base_path,
    filepath,
    verbose,
  } = Config::parse();

  // reads word list at 'filepath'
  let words = read_lines_to_vec(filepath.into());

  let resolved_urls = resolver::resolve_paths(base_path.to_string(), words, verbose).unwrap();

  println!("Resolved urls are:");
  for url in resolved_urls {
    println!("\t* {}", url);
  }
}
