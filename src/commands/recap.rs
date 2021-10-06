//! Posts the recap image

const RECAP_URL: &str = "https://i.imgur.com/LPKt2uF.png";

/// It's time for the recap
pub fn recap() -> String {
    format!("It's time for the r-r-r-r-recap!\n{}", RECAP_URL)
}
