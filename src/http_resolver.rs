use reqwest::{blocking as client, StatusCode};

pub fn is_url_resolved(url: &str) -> bool {
  client::get(url).map_or(false, |r| r.status() != StatusCode::from_u16(404).unwrap())
}

pub fn format_url(base_path: &str, tail: &str) -> String {
  format!("{}/{}", base_path, tail)
}
