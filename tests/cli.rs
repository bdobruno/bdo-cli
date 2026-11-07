use std::process::{Command, Output};

fn bdo(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_bdo"))
        .args(args)
        .output()
        .unwrap()
}

/// REQ-001
#[test]
fn add_without_categories_fails_with_usage() {
    let output = bdo(&["add"]);

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Usage: bdo add <category>..."));
}

/// REQ-001, REQ-014
#[test]
fn add_global_without_categories_fails_with_usage() {
    let output = bdo(&["add", "--global"]);

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Usage: bdo add <category>... [--global]"));
}

/// REQ-013
#[test]
fn add_outside_a_git_repo_fails_and_writes_nothing() {
    let dir = tempfile::TempDir::new().unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_bdo"))
        .args(["add", "core"])
        .current_dir(dir.path())
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("not a git repository — run bdo add inside a project"));
    assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 0);
}

/// REQ-011
#[test]
fn help_lists_the_add_command() {
    let output = bdo(&["help"]);

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("bdo add <category>..."));
}

/// REQ-018
#[test]
fn help_lists_skills_command_and_global_flag() {
    let output = bdo(&["help"]);

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("bdo add <category>... [--global]"));
    assert!(stdout.contains("bdo skills"));
}
