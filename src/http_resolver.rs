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
