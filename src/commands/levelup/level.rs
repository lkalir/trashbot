//! Modify campaign levels

use super::{save_db, MySmolStr, LEVEL_MAP};
use chrono;
use log::warn;
use serenity::model::id::GuildId;
use smol_str::SmolStr;

/// Set your levels
pub async fn level(id: GuildId, amt: Option<u8>, campaign: SmolStr) -> String {
    let now = chrono::offset::Local::now();
    let level = *LEVEL_MAP
        .lock()
        .await
        .entry((id, MySmolStr(campaign.clone())))
        .and_modify(|(l, d)| {
            *l = amt.unwrap_or(*l + 1);
            *d = now;
        })
        .or_insert_with(|| (amt.unwrap_or(1), now));

    if let Err(why) = save_db(&*LEVEL_MAP.lock().await).await {
        warn!("Fug: {}", why);
        "Failed to set level".to_string()
    } else {
        format!("'{}' is now level {}", campaign, level.0)
    }
}
