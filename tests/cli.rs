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

/// REQ-011
#[test]
fn help_lists_the_add_command() {
    let output = bdo(&["help"]);

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("bdo add <category>..."));
}
