//! Summarize recent changes

use graphql_client::{GraphQLQuery, Response};
use log::warn;
use serde::Serialize;
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
pub async fn wha_happun(cnt: Option<i64>) -> Result<String, Box<dyn std::error::Error>> {
    let amt = cnt.unwrap_or(5);
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

        Ok(update_msg)
    } else {
        warn!("Missing data from query");
        Err("Missing data from query".into())
    }
}
