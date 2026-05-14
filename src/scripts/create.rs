use std::fs;
use std::path::Path;

use super::{run_command, ScriptResult};

pub fn run(repo_name: &str) -> ScriptResult {
    if repo_name.is_empty() {
        return Err("repository name is required".into());
    }

    if Path::new(repo_name).exists() {
        return Err(format!("directory '{repo_name}' already exists").into());
    }

    fs::create_dir(repo_name)?;
    println!("Created directory: {repo_name}");

    std::env::set_current_dir(repo_name)?;
    run_command("git", &["init"])?;
    fs::write("README.md", format!("# {repo_name}\n"))?;
    run_command("git", &["add", "."])?;
    run_command("git", &["commit", "-m", "Initial commit"])?;
    run_command(
        "gh",
        &[
            "repo",
            "create",
            repo_name,
            "--public",
            "--source=.",
            "--remote=origin",
        ],
    )?;

    println!("Repository '{repo_name}' created successfully.");
    Ok(())
}
