//! Should clap-clap

use lazy_regex::regex;
use smol_str::SmolStr;

/// Detects whether to clap-clap at a message
pub fn is_lap(msg: &str) -> Option<SmolStr> {
    regex!(r"(\w*lap)")
        .captures(msg)
        .map(|caps| caps.get(1).unwrap().as_str().into())
}
