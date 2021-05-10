//! Bird up!

use lazy_regex::regex;

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

/// Detects whether a message warrants a "bird up"
pub fn is_bird(msg: &str) -> bool {
    regex!(r"\b(?:bird|burd|berd|birb|birdo|chirido|dee|feathered biped|avian|chirp|chirrido|torin|jackie|jaqueline|tit)[sz]*\b").is_match(msg)
}
