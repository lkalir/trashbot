//! Should clap-clap

use once_cell::sync::OnceCell;
use regex::Regex;

static RE: OnceCell<Regex> = OnceCell::new();

/// Detects whether to clap-clap at a message
pub fn is_lap(msg: &str) -> Option<String> {
    let r = RE.get_or_init(|| Regex::new(r"(\w*lap)").unwrap());
    r.captures(msg)
        .map(|caps| caps.get(1).unwrap().as_str().to_owned())
}
