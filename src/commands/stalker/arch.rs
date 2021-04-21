//! Arch logo spammer

/// List of arch phrases
static ARCH: &[&str] = &["arch", "btw"];

/// Detects whether a message warrants an arch reaction
pub fn is_arch(msg: &str) -> bool {
    ARCH.iter()
        .find_map(|b| msg.contains(b).then(|| ()))
        .is_some()
}
