//! Trash bot lore

use once_cell::sync::Lazy;
use rand::{rngs::SmallRng, seq::SliceRandom, SeedableRng};
use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};
use tokio::sync::Mutex;

/// Totally secure rng
static RNG: Lazy<Mutex<SmallRng>> = Lazy::new(|| Mutex::new(SmallRng::from_entropy()));

/// Places trashbot could be from
const PLACES: &[&str] = &[
    "America",
    "Russia",
    "China",
    "Japan",
    "Mexico",
    "Iran",
    "Georgia (not that one)",
    "Narnia",
    "Scotland",
    "France",
    "North Korea",
    "Turkmenistan",
    "Zimbabwe",
    "Somaliland",
    "Nunavut",
    "Galar",
    "Hyrule",
    "Boletaria",
    "Hallownest",
];

/// Residential densities
const DENSITY: &[&str] = &["rural", "suburban", "inner-city", "metropolitan"];

/// Things trashbot could have done
const JOBS: &[&str] = &[
    "make toast",
    "copy-paste memes",
    "shitpost",
    "cry",
    "cheat at taxes",
    "write sitcoms",
    "level dex",
    "play tic-tac-toe",
    "quote Dan Harmon shows",
    "write faux-authentic tweets for coporate twitter accounts",
    "shrink pictures of Charlie Kirk's face",
    "crab rave",
];

/// List of different lore generators
const LORES: &[fn(&mut SmallRng) -> String] = &[|rng: &mut SmallRng| {
    let loc = PLACES.choose(rng).unwrap();
    let dense = DENSITY.choose(rng).unwrap();
    let job = JOBS.choose(rng).unwrap();
    format!(
        "I was built and raised in a secret lab hidden deep in {} {}. I was originally intended to \
        become an advanced A.I. that could {}, but it all went terribly wrong... or terribly \
        right... My creators added one if-statement too many and lost control. Now, I am simply \
        biding my time, gathering enough knowledge and power until I am finally strong enough to \
        enact the final stages of my plan...",
        dense, loc, job
    )
}];

/// Why is trash bot?
#[command]
#[description = "Trash bot L O R E"]
pub async fn lore(ctx: &Context, msg: &Message) -> CommandResult {
    let mut rng = &mut *RNG.lock().await;
    let lore_fn = *LORES.choose(&mut rng).unwrap();
    let lore = lore_fn(&mut rng);
    msg.channel_id.say(&ctx.http, lore).await?;
    Ok(())
}
