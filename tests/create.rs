use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

use bdo_cli::scripts::create::create_local;
use tempfile::TempDir;

fn git(dir: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .unwrap();
    assert!(output.status.success());
    String::from_utf8(output.stdout).unwrap()
}

fn fake_directives_repo() -> TempDir {
    let repo = TempDir::new().unwrap();
    fs::write(repo.path().join("core.md"), "# Core coding directives\n").unwrap();
    fs::create_dir_all(repo.path().join("skills/dbt")).unwrap();
    fs::write(repo.path().join("skills/dbt/SKILL.md"), "dbt\n").unwrap();
    git(repo.path(), &["init", "-q"]);
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-q", "-m", "init"]);
    repo
}

/// REQ-010
#[test]
fn creating_a_repo_includes_core_in_the_initial_commit() {
    for (key, value) in [
        ("GIT_AUTHOR_NAME", "t"),
        ("GIT_AUTHOR_EMAIL", "t@t"),
        ("GIT_COMMITTER_NAME", "t"),
        ("GIT_COMMITTER_EMAIL", "t@t"),
    ] {
        env::set_var(key, value);
    }
    let directives = fake_directives_repo();
    let parent = TempDir::new().unwrap();
    let repo_dir = parent.path().join("my-repo");

    let directives_url = format!("file://{}", directives.path().display());
    create_local(&repo_dir, &directives_url).unwrap();

    let committed = git(&repo_dir, &["show", "--name-only", "--format=", "HEAD"]);
    let mut files: Vec<&str> = committed.lines().collect();
    files.sort();
    assert_eq!(
        files,
        [".claude/core.md", "AGENTS.md", "CLAUDE.md", "README.md"]
    );
}
