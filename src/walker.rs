use std::{
  sync::{Arc, Mutex},
  thread,
};

use crate::http_resolver::{self, format_url};

/// Multithreaded impl of web directory traversal
/// Note that this impl doesnt walk the web recursively yet but only in 1 depth
pub struct Walker {
  num_threads: usize,
  verbose: bool,
}

const DEFAULT_NUM_THREADS: usize = 4;

impl Walker {
  /// Constructs a web dir walker  given the number of threads to use (fallback to 4 if None) and
  /// prints logs if verbose is true
  pub fn new(num_threads: Option<usize>, verbose: bool) -> Walker {
    Self {
      num_threads: num_threads.unwrap_or(DEFAULT_NUM_THREADS),
      verbose,
    }
  }

  /// below comments will tell you the steps and teach you about multithreaded rust (and smart pointers)
  /// Attempts to resolve the path list on the base_path
  /// Returns the potential vulnerable paths
  pub fn traverse_all(self, base_path: String, paths: Vec<String>) -> Result<Vec<String>, ()> {
    // using Rust's smart pointers here hehe:)...
    // Arc for thread safe multiple ownership so we can access the same allocated 'resolved_urls'
    // accross multiple thread
    // Mutex to allow access to 'resolved_urls' from one thread at a time
    let resolved_urls = Arc::new(Mutex::new(Vec::new()));
    let units_per_thread = paths.len() / self.num_threads;

    // store JoinHandle for each spawned thread so we can wait for their completion later
    let handles = (0..self.num_threads)
      .map(|i| {
        let from_idx = i * units_per_thread;
        let to_idx = from_idx + units_per_thread;
        // portion of the paths for a thread
        let paths = paths[from_idx..to_idx].to_vec();

        let base_path = base_path.clone();

        // access the resolved_urls to use in the below thread
        let resolved_urls = Arc::clone(&resolved_urls);

        // spawn a thread, returning its JoinHandle
        thread::spawn(move || {
          for path in paths {
            let url = format_url(&base_path, &path);
            if self.verbose {
              println!("resolving: {}", url);
            }
            if http_resolver::is_url_resolved(&url) {
              // acquire the resolved_urls so only one thread have access to it at this time
              // mutex also provide interior mutability like RefCell, but unlike RefCell, it is
              // threadsafe
              let mut resolved_urls = resolved_urls.lock().unwrap();

              // if the url is resolved then push it inside the vec
              resolved_urls.push(url);
            }
          }
        })
      })
      .collect::<Vec<_>>();

    // the spawned thread is "detached" which means that there is no way to know when the spawned
    // thread completes or otherwise terminates, call .join() to wait for the completion of the
    // spawned thread
    for handle in handles {
      handle.join().unwrap();
    }

    let resolved_urls = resolved_urls.lock().unwrap();
    Ok(resolved_urls.to_vec())
  }
}
