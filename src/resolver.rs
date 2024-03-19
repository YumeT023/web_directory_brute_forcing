use std::{
  sync::{Arc, Mutex},
  thread,
};

use reqwest::{blocking as client, StatusCode};

/// Attempt to resolve the specified HTTP URL. In HTTP terms, a 404 status code signifies 'Not
/// Found', indicating that the client was unable to locate the requested resource.
pub fn is_url_resolved(url: &str) -> bool {
  client::get(url).map_or(false, |r| r.status() != StatusCode::from_u16(404).unwrap())
}

/// Joins base_path and tail resulting in a correct url
/// if tail is '/', no double '/' will be appended
pub fn format_url(base_path: &str, tail: &str) -> String {
  match tail {
    "/" => base_path.to_string(),
    _ => format!("{}/{}", base_path, tail),
  }
}

/// Multithreaded impl of web directory traversal
/// Note that this impl doesnt walk the web recursively yet but only in 1 depth

/// Attempts to resolve the path list on the base_path
/// Returns the potential vulnerable paths
pub fn resolve_paths(
  base_path: String,
  paths: Vec<String>,
  verbose: bool,
) -> Result<Vec<String>, ()> {
  // [thread_safe, multiple ownership] vec to store the resolved_urls
  let resolved_urls = Arc::new(Mutex::new(Vec::new()));

  // get the available cpu on the system
  let cpus = num_cpus::get();

  let units_per_thread = paths.len() / cpus;

  // store JoinHandle for each spawned thread so we can wait for their completion later
  let handles = (0..cpus)
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
          if verbose {
            println!("resolving: {}", &path);
          }
          if is_url_resolved(&url) {
            // acquire the resolved_urls so only one thread have access to it at this time
            // mutex also provide interior mutability like RefCell, but unlike RefCell, it is
            // threadsafe
            let mut resolved_urls = resolved_urls.lock().unwrap();

            // if the url is resolved then push it inside the vec
            resolved_urls.push(path);
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
