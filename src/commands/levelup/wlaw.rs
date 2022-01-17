//! Checks campaign level

use super::{MySmolStr, LEVEL_MAP};
use serenity::model::id::GuildId;
use smol_str::SmolStr;

/// "What level are we?"
pub async fn wlaw(id: GuildId, campaign: Option<SmolStr>) -> String {
    if let Some(campaign) = campaign {
        LEVEL_MAP
            .lock()
            .await
            .get(&(id, MySmolStr(campaign.clone())))
            .map_or_else(
                || format!("No such campaign '{}", campaign),
                |(level, date)| {
                    format!(
                        "'{campaign}' is level {level} since {}",
                        date.date().format("%m/%d/%Y")
                    )
                },
            )
    } else {
        let camps: Vec<String> = LEVEL_MAP
            .lock()
            .await
            .iter()
            .filter_map(|((guild, campaign), level)| {
                if *guild == id {
                    Some((campaign, level))
                } else {
                    None
                }
            })
            .map(|(campaign, (level, date))| {
                format!(
                    "'{}' is level {level} since {}",
                    campaign.0,
                    date.date().format("%m/%d/%Y")
                )
            })
            .collect();
        let camps = camps.join("\n");

        if camps.is_empty() {
            "No campaigns tracked in server".to_string()
        } else {
            camps
        }
    }
}
