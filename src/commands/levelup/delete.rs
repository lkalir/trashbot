//! Delete campaign levels

use super::{save_db, MySmolStr, LEVEL_MAP};
use log::warn;
use serenity::model::id::GuildId;
use smol_str::SmolStr;

/// Delete campaign
pub async fn delete(id: GuildId, campaign: SmolStr) -> String {
    if LEVEL_MAP
        .lock()
        .await
        .remove_entry(&(id, MySmolStr(campaign.clone())))
        .is_some()
    {
        if let Err(why) = save_db(&*LEVEL_MAP.lock().await).await {
            warn!("Failed to delete campaign {:?}", why);
            "memes".to_string()
        } else {
            format!("Deleted '{}'", campaign)
        }
    } else {
        format!("No such campaign '{}'", campaign)
    }
}
