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
    "chirrido",
    "torin",
    "jackie",
    "jaqueline",
];

/// Detects whether a message warrants a "bird up"
pub fn is_bird(msg: &str) -> bool {
    BIRDS
        .iter()
        .find_map(|b| msg.contains(b).then(|| ()))
        .is_some()
}
