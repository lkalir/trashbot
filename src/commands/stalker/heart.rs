//! Thank people for their kind words

use lazy_regex::regex;

/// Detects if a comment is praising trashbot
pub fn is_heart(msg: &str) -> bool {
    regex!(r"\bthank[sz]*|thx|right|correct|love|yay+|hooray|hurrah|huzzah|ty|cool|yas+\b")
        .is_match(msg)
        && regex!(r"\btrash[-_ ]*bot\b").is_match(msg)
}
