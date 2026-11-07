use std::fs;
use std::path::Path;

use super::{add, run_command, ScriptResult};

pub fn run(repo_name: &str) -> ScriptResult {
    if repo_name.is_empty() {
        return Err("repository name is required".into());
    }

    let repo_dir = Path::new(repo_name);
    create_local(repo_dir, add::DIRECTIVES_REPO)?;
    publish(repo_dir, repo_name)?;

    println!("Repository '{repo_name}' created successfully.");
    Ok(())
}

pub fn create_local(repo_dir: &Path, directives_repo: &str) -> ScriptResult {
    if repo_dir.exists() {
        return Err(format!("directory '{}' already exists", repo_dir.display()).into());
    }

    fs::create_dir(repo_dir)?;
    println!("Created directory: {}", repo_dir.display());

    let dir = repo_dir.to_str().ok_or("invalid repository path")?;
    run_command("git", &["-C", dir, "init"])?;
    fs::write(
        repo_dir.join("README.md"),
        format!("# {}\n", dir_name(repo_dir)),
    )?;
    add::add_from_repo(directives_repo, repo_dir, &["core"])?;
    run_command("git", &["-C", dir, "add", "."])?;
    run_command("git", &["-C", dir, "commit", "-m", "Initial commit"])?;

    Ok(())
}

fn publish(repo_dir: &Path, repo_name: &str) -> ScriptResult {
    let source = format!("--source={}", repo_dir.display());
    run_command(
        "gh",
        &[
            "repo",
            "create",
            repo_name,
            "--public",
            &source,
            "--remote=origin",
        ],
    )
}

fn dir_name(repo_dir: &Path) -> String {
    repo_dir
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default()
}
