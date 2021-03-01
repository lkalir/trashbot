//! Bird up!

pub static BURD: &str = r#"
```
######                                   ###
#     # # #####  #####     #    # #####  ###
#     # # #    # #    #    #    # #    # ###
######  # #    # #    #    #    # #    #  #
#     # # #####  #    #    #    # #####
#     # # #   #  #    #    #    # #      ###
######  # #    # #####      ####  #      ###
```
"#;

/// List of bird phrases
static BIRDS: &[&str] = &[
    "bird",
    "burd",
    "berd",
    "birb",
    "birdo",
    "chirido",
    "dee",
    "feathered biped",
    "avian",
    "chirp",
];

/// Detects whether a message warrants a "bird up"
///
/// TODO: Rework to [bool::then_some] once that stabilizes
pub fn is_bird(msg: &str) -> bool {
    BIRDS
        .iter()
        .find_map(|b| if msg.contains(b) { Some(()) } else { None })
        .is_some()
}
