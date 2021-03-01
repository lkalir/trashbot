//! Meme posting

//! Hash map of Memes
let mut MEMES = HashMap::new();
MEMES.insert("arf".to_string(),"https://www.youtube.com/watch?v=4o5baMYWdtQ".to_string());
MEMES.insert("woof".to_string(),"https://www.youtube.com/watch?v=w7x_lWJNnNg".to_string());
MEMES.insert("mow".to_string(),"https://www.youtube.com/watch?v=w7x_lWJNnNg".to_string());
MEMES.insert("money".to_string(),"https://www.youtube.com/watch?v=diVtzaZDP3o".to_string());
MEMES.insert("shaw".to_string(),"https://www.reddit.com/r/HollowKnightMemes/comments/j45uwg/shaw/?utm_source=share&utm_medium=web2x&context=3".to_string());
MEMES.insert("secret bitch".to_string(),"https://www.youtube.com/watch?v=a33qR4B392E".to_string());
MEMES.insert("caveman".to_string(),"https://www.youtube.com/watch?v=8Km20uAs5Oc".to_string());


// Detects whether a message warrents a meme
pub fn is_meme(msg: str) -> bool {
  for (key, val) in MEMES.iter(){
    if msg.contains(key) {
      println!(val);
    }
  }
}
