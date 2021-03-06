//! Level tracking tools
pub mod delete;
pub mod level;
pub mod wlaw;

use log::info;
use once_cell::sync::Lazy;
use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};
use serenity::{framework::standard::CommandResult, model::id::GuildId};
use smol_str::SmolStr;
use std::{collections::HashMap, env, io::ErrorKind};
use tokio::{io::AsyncWriteExt, sync::Mutex};

/// Wrapper type around SmolStr
///
/// SmolStr doesn't implement serde, so we have to do it ourselves
#[derive(Hash, PartialEq, Eq, Clone)]
struct MySmolStr(SmolStr);

impl Serialize for MySmolStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.0.as_str())
    }
}

struct StrVisitor;

impl<'de> Visitor<'de> for StrVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v.into())
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v)
    }
}

impl<'de> Deserialize<'de> for MySmolStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = deserializer.deserialize_str(StrVisitor)?;
        Ok(MySmolStr(s.into()))
    }
}

static LEVEL_MAP: Lazy<Mutex<HashMap<(GuildId, MySmolStr), u8>>> = Lazy::new(|| {
    let path = env::var("DB_LOCATION").unwrap_or_else(|_| "campaigndb.json".to_string());
    let m = if let Ok(file) = std::fs::OpenOptions::new().read(true).open(&path) {
        if let Ok(db) = serde_json::from_reader(file) {
            info!("Loading level database from '{}'", path);
            LevelMapRecord::vec_to_map(db)
        } else {
            HashMap::new()
        }
    } else {
        HashMap::new()
    };
    Mutex::new(m)
});

/// Intermediary data type  for (de)serialization
#[derive(Serialize, Deserialize)]
struct LevelMapRecord {
    campaign: MySmolStr,
    id: GuildId,
    level: u8,
}

impl LevelMapRecord {
    fn new(id: GuildId, campaign: MySmolStr, level: u8) -> Self {
        Self {
            campaign,
            id,
            level,
        }
    }

    /// Prepares level database for serialization to disk
    fn map_to_vec(hm: &HashMap<(GuildId, MySmolStr), u8>) -> Vec<Self> {
        hm.iter()
            .map(|((id, campaign), level)| Self::new(*id, campaign.clone(), *level))
            .collect()
    }

    /// Converts deserialized data back to in-memory db
    fn vec_to_map(v: Vec<Self>) -> HashMap<(GuildId, MySmolStr), u8> {
        let mut m = HashMap::new();
        v.iter().for_each(|record| {
            m.insert((record.id, record.campaign.clone()), record.level);
        });
        m
    }
}

/// Saves campaign db to disk
async fn save_db(hm: &HashMap<(GuildId, MySmolStr), u8>) -> CommandResult {
    let c = LevelMapRecord::map_to_vec(hm);
    let db = serde_json::to_string_pretty(&c)?;
    let path = env::var("DB_LOCATION").unwrap_or_else(|_| "campaigndb.json".to_string());

    // File not found just means this is first call
    if let Err(e) = tokio::fs::remove_file(&path).await {
        if e.kind() != ErrorKind::NotFound {
            return Err(e.into());
        }
    }

    let mut file = tokio::fs::OpenOptions::new()
        .write(true)
        .append(false)
        .create(true)
        .open(path)
        .await?;
    file.write_all(db.as_bytes()).await?;

    Ok(())
}
