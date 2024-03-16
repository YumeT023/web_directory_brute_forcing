use std::{
  sync::{Arc, Mutex},
  thread,
};

use crate::http_resolver::{self, format_url};

pub struct Walker {
  num_threads: usize,
  verbose: bool,
}

const DEFAULT_NUM_THREADS: usize = 4;

impl Walker {
  pub fn new(num_threads: Option<usize>, verbose: bool) -> Walker {
    Self {
      num_threads: num_threads.unwrap_or(DEFAULT_NUM_THREADS),
      verbose,
    }
  }

  pub fn traverse_all(self, base_path: String, paths: Vec<String>) -> Result<Vec<String>, ()> {
    let resolved_urls = Arc::new(Mutex::new(Vec::new()));
    let units_per_thread = paths.len() / self.num_threads;

    let handles = (0..self.num_threads)
      .map(|i| {
        let from_idx = i * units_per_thread;
        let to_idx = from_idx + units_per_thread;

        let resolved_urls = Arc::clone(&resolved_urls);
        let paths = paths[from_idx..to_idx].to_vec();
        let base_path = base_path.clone();

        thread::spawn(move || {
          for path in paths {
            let url = format_url(&base_path, &path);
            if self.verbose {
              println!("resolving: {}", url);
            }
            if http_resolver::is_url_resolved(&url) {
              let mut resolved_urls = resolved_urls.lock().unwrap();
              resolved_urls.push(url);
            }
          }
        })
      })
      .collect::<Vec<_>>();

    for handle in handles {
      handle.join().unwrap();
    }

    let resolved_urls = resolved_urls.lock().unwrap();
    Ok(resolved_urls.to_vec())
  }
}
