//! Thank people for their kind words

/// List of good words
static POSITIVES: &[&str] = &[
    "thank", "thx", "right", "correct", "love", "yay", "hooray", "hurrah", "huzzah", "ty", "cool",
];

/// List of possible names for trashbot
static TRASHBOT_NAMES: &[&str] = &["trashbot", "trash-bot", "trash bot", "trash_bot"];

/// Detects if a comment is praising trashbot
pub fn is_heart(msg: &str) -> bool {
    POSITIVES
        .iter()
        .find_map(|b| msg.contains(b).then(|| ()))
        .is_some()
        && TRASHBOT_NAMES
            .iter()
            .find_map(|b| msg.contains(b).then(|| ()))
            .is_some()
}
