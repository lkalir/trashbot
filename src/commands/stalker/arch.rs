//! Arch logo spammer

/// List of arch phrases
static ARCH: &[&str] = &["arch", "btw"];

/// Detects whether a message warrants an arch reaction
///
/// TODO: Rework to [bool::then_some] once that stabilizes
pub fn is_arch(msg: &str) -> bool {
    ARCH.iter()
        .find_map(|b| if msg.contains(b) { Some(()) } else { None })
        .is_some()
}
