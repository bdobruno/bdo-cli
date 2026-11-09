use std::error::Error;

use serde_json::Value;

use super::github::{GhCli, GitHub, NotFound};
use super::ScriptResult;

pub const RULESET: &str = include_str!("../rulesets/protect-main.json");

pub fn run() -> ScriptResult {
    let github = GhCli;
    let repo = github.current_repo()?;
    apply(&github, &repo)?;

    println!("Default branch of '{repo}' is now protected.");
    Ok(())
}

pub fn apply(github: &impl GitHub, repo: &str) -> ScriptResult {
    let name = ruleset_name(RULESET)?;
    let existing = github.list_rulesets(repo)?;

    let saved = match find_ruleset_id(&existing, &name)? {
        Some(id) => github.update_ruleset(repo, id, RULESET),
        None => github.create_ruleset(repo, RULESET),
    };

    match saved {
        Err(error) if error.is::<NotFound>() => {
            Err(missing_admin_message(repo, &github.current_user()?).into())
        }
        other => other,
    }
}

fn missing_admin_message(repo: &str, user: &str) -> String {
    format!(
        "you need admin access to {repo} to protect its default branch, \
         but gh is logged in as {user}. Switch accounts with \
         `gh auth switch` or `gh auth login`"
    )
}

fn ruleset_name(ruleset: &str) -> Result<String, Box<dyn Error>> {
    let value: Value = serde_json::from_str(ruleset)?;
    let name = value["name"].as_str().ok_or("ruleset has no name")?;

    Ok(name.to_string())
}

fn find_ruleset_id(rulesets: &str, name: &str) -> Result<Option<u64>, Box<dyn Error>> {
    let rulesets: Vec<Value> = serde_json::from_str(rulesets)?;

    Ok(rulesets
        .iter()
        .find(|ruleset| ruleset["name"] == name)
        .and_then(|ruleset| ruleset["id"].as_u64()))
}
