//! Meme posting

use std::collections::HashMap;

// Detects whether a message warrents a meme
pub fn is_meme(msg: &str) -> Option<String> {
  // Hash map of Memes
  let mut memes = HashMap::new();
  memes.insert("arf".to_string(),"https://www.youtube.com/watch?v=4o5baMYWdtQ".to_string());
  memes.insert("woof".to_string(),"https://www.youtube.com/watch?v=83m261lAlrs".to_string());
  memes.insert("mow".to_string(),"https://www.youtube.com/watch?v=w7x_lWJNnNg".to_string());
  memes.insert("money".to_string(),"https://www.youtube.com/watch?v=diVtzaZDP3o".to_string());
  memes.insert("shaw".to_string(),"https://www.reddit.com/r/HollowKnightMemes/comments/j45uwg/shaw/?utm_source=share&utm_medium=web2x&context=3".to_string());
  memes.insert("secret bitch".to_string(),"https://www.youtube.com/watch?v=a33qR4B392E".to_string());
  memes.insert("caveman".to_string(),"https://www.youtube.com/watch?v=8Km20uAs5Oc".to_string());
  

  memes.iter().find_map(|(key,val)| {
    if msg.contains(key) {
      Some(val.clone())
    } else {
      None
    }
  })
}
