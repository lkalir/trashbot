//! Summarize recent changes

use graphql_client::{GraphQLQuery, Response};
use log::warn;
use serde::Serialize;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};
use smol_str::SmolStr;
use std::env;

// Procedural macro power!
#[derive(GraphQLQuery, Serialize)]
#[graphql(
    schema_path = "src/commands/wha_happun/schema.graphql",
    query_path = "src/commands/wha_happun/query.graphql"
)]
struct CommitQuery;

/// Messages the channel with the headlines of the latest commits to master
#[command]
#[description = "What's new in trashbot."]
#[min_args(0)]
#[max_args(1)]
#[usage = "[number of commits: DEFAULT 5]"]
pub async fn wha_happun(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let amt = args.clone().single().unwrap_or(5);
    let q = CommitQuery::build_query(commit_query::Variables { num: Some(amt) });

    // All the data we're after is all public, yet Github's api requires a token anyways
    let tok = env::var("GITHUB_TOKEN")?;

    let client = reqwest::Client::builder()
        .user_agent("graphql-rust/0.10.0")
        .build()?;
    let res = client
        .post("https://api.github.com/graphql")
        .bearer_auth(tok)
        .json(&q)
        .send()
        .await?;

    res.error_for_status_ref()?;

    let body: Response<commit_query::ResponseData> = res.json().await?;

    // THIS is the power of idiomatic rust. Fear its destructuring power!
    if let Response::<commit_query::ResponseData> {
        data:
            Some(commit_query::ResponseData {
                repository:
                    Some(commit_query::CommitQueryRepository {
                        object:
                            Some(commit_query::CommitQueryRepositoryObject::Commit(
                                commit_query::CommitQueryRepositoryObjectOnCommit {
                                    history:
                                        commit_query::CommitQueryRepositoryObjectOnCommitHistory {
                                            nodes: Some(nodes),
                                        },
                                },
                            )),
                    }),
            }),
        ..
    } = body
    {
        // Build list of commits
        let mut updates: Vec<SmolStr> = nodes
            .iter()
            .filter_map(|node| node.as_ref())
            .map(|node| format!("* {} - {}", node.abbreviated_oid, node.message_headline).into())
            .collect();

        // Insert some discord formatting
        updates.insert(0, "```".into());
        updates.insert(0, "Here's what I learned to do recently!".into());
        updates.push("```".into());
        let update_msg = updates.join("\n");

        msg.channel_id.say(&ctx.http, update_msg).await?;
        Ok(())
    } else {
        warn!("Missing data from query");
        Err("Missing data from query".into())
    }
}
