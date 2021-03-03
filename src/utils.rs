//! Functions that are generally useful

use log::{info, warn};
use once_cell::sync::Lazy;
use serenity::{
    client::Context,
    http::CacheHttp,
    model::{id::GuildId, misc::EmojiIdentifier},
};
use std::collections::HashMap;
use tokio::sync::Mutex;

/// Cache of emojis from different servers
///
/// Need to use an async-compliant mutex instead of the standard library since this is used in
/// async contexts
static MOJI_CACHE: Lazy<Mutex<HashMap<(GuildId, String), EmojiIdentifier>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Looks up an emoji by name in a guild, adding it to a cache if not present
pub async fn lookup_and_cache(
    guild: GuildId,
    ctx: &Context,
    name: &str,
) -> Option<EmojiIdentifier> {
    let moj = &mut *MOJI_CACHE.lock().await;
    match moj.get(&(guild, name.to_string())) {
        Some(e) => Some(e.clone()),
        None => {
            let mojis = ctx
                .cache
                .guild(guild)
                .await
                .unwrap()
                .emojis(ctx.http())
                .await
                .unwrap();
            if let Some(found_moji) = mojis.iter().find(|moji| moji.name == name) {
                let id = EmojiIdentifier {
                    animated: found_moji.animated,
                    id: found_moji.id,
                    name: found_moji.name.clone(),
                };
                info!("Caching '{}' for guild {}", name, guild);
                moj.insert((guild, name.to_string()), id.clone());
                Some(id)
            } else {
                warn!("Failed to lookup '{}' for guild {}", name, guild);
                None
            }
        }
    }
}
