use std::fs;
use std::path::Path;
use std::process::Command;

use bdo_cli::scripts::add::{add_from, add_from_repo};
use tempfile::TempDir;

fn git(dir: &Path, args: &[&str]) {
    let status = Command::new("git")
        .args(args)
        .current_dir(dir)
        .status()
        .unwrap();
    assert!(status.success());
}

fn fake_directives() -> TempDir {
    let dir = TempDir::new().unwrap();
    let root = dir.path();
    fs::create_dir_all(root.join("skills/rust/rust-testing")).unwrap();
    fs::create_dir_all(root.join("skills/rust/rust-best-practices/references")).unwrap();
    fs::create_dir_all(root.join("skills/dbt")).unwrap();
    fs::write(root.join("core.md"), "# Core coding directives\n").unwrap();
    fs::write(
        root.join("skills/rust/rust-testing/SKILL.md"),
        "rust testing\n",
    )
    .unwrap();
    fs::write(
        root.join("skills/rust/rust-best-practices/SKILL.md"),
        "rust bp\n",
    )
    .unwrap();
    fs::write(
        root.join("skills/rust/rust-best-practices/references/chapter_01.md"),
        "chapter 1\n",
    )
    .unwrap();
    fs::write(root.join("skills/dbt/SKILL.md"), "dbt\n").unwrap();
    dir
}

/// REQ-002
#[test]
fn adding_core_copies_core_md_into_claude_dir() {
    let source = fake_directives();
    let target = TempDir::new().unwrap();

    add_from(source.path(), target.path(), &["core"]).unwrap();

    let copied = fs::read_to_string(target.path().join(".claude/core.md")).unwrap();
    assert_eq!(copied, "# Core coding directives\n");
}

/// REQ-003
#[test]
fn adding_core_creates_claude_md_importing_core() {
    let source = fake_directives();
    let target = TempDir::new().unwrap();

    add_from(source.path(), target.path(), &["core"]).unwrap();

    let claude_md = fs::read_to_string(target.path().join("CLAUDE.md")).unwrap();
    assert_eq!(claude_md, "@.claude/core.md\n");
}

/// REQ-003
#[test]
fn adding_core_appends_import_to_existing_claude_md() {
    let source = fake_directives();
    let target = TempDir::new().unwrap();
    fs::write(target.path().join("CLAUDE.md"), "# My project\n").unwrap();

    add_from(source.path(), target.path(), &["core"]).unwrap();

    let claude_md = fs::read_to_string(target.path().join("CLAUDE.md")).unwrap();
    assert_eq!(claude_md, "# My project\n\n@.claude/core.md\n");
}

/// REQ-003
#[test]
fn adding_core_twice_does_not_duplicate_import() {
    let source = fake_directives();
    let target = TempDir::new().unwrap();

    add_from(source.path(), target.path(), &["core"]).unwrap();
    add_from(source.path(), target.path(), &["core"]).unwrap();

    let claude_md = fs::read_to_string(target.path().join("CLAUDE.md")).unwrap();
    assert_eq!(claude_md, "@.claude/core.md\n");
}

/// REQ-004
#[test]
fn adding_core_creates_agents_md_pointing_to_core() {
    let source = fake_directives();
    let target = TempDir::new().unwrap();

    add_from(source.path(), target.path(), &["core"]).unwrap();

    let agents_md = fs::read_to_string(target.path().join("AGENTS.md")).unwrap();
    assert_eq!(
        agents_md,
        "Follow the coding directives in `.claude/core.md`.\n"
    );
}

/// REQ-004
#[test]
fn adding_core_twice_does_not_duplicate_agents_pointer() {
    let source = fake_directives();
    let target = TempDir::new().unwrap();
    fs::write(target.path().join("AGENTS.md"), "# Agents\n").unwrap();

    add_from(source.path(), target.path(), &["core"]).unwrap();
    add_from(source.path(), target.path(), &["core"]).unwrap();

    let agents_md = fs::read_to_string(target.path().join("AGENTS.md")).unwrap();
    assert_eq!(
        agents_md,
        "# Agents\n\nFollow the coding directives in `.claude/core.md`.\n"
    );
}

/// REQ-005
#[test]
fn adding_a_category_copies_each_of_its_skills_into_claude_skills() {
    let source = fake_directives();
    let target = TempDir::new().unwrap();

    add_from(source.path(), target.path(), &["rust"]).unwrap();

    let skills = target.path().join(".claude/skills");
    assert_eq!(
        fs::read_to_string(skills.join("rust-testing/SKILL.md")).unwrap(),
        "rust testing\n"
    );
    assert_eq!(
        fs::read_to_string(skills.join("rust-best-practices/SKILL.md")).unwrap(),
        "rust bp\n"
    );
    assert_eq!(
        fs::read_to_string(skills.join("rust-best-practices/references/chapter_01.md")).unwrap(),
        "chapter 1\n"
    );
}

/// REQ-005
#[test]
fn adding_a_skill_category_does_not_add_core() {
    let source = fake_directives();
    let target = TempDir::new().unwrap();

    add_from(source.path(), target.path(), &["rust"]).unwrap();

    assert!(!target.path().join(".claude/core.md").exists());
    assert!(!target.path().join("CLAUDE.md").exists());
}

/// REQ-006
#[test]
fn adding_a_category_that_is_itself_a_skill_copies_it_whole() {
    let source = fake_directives();
    let target = TempDir::new().unwrap();

    add_from(source.path(), target.path(), &["dbt"]).unwrap();

    let skill_md = fs::read_to_string(target.path().join(".claude/skills/dbt/SKILL.md")).unwrap();
    assert_eq!(skill_md, "dbt\n");
}

/// REQ-007
#[test]
fn re_adding_a_skill_replaces_the_existing_copy() {
    let source = fake_directives();
    let target = TempDir::new().unwrap();
    let existing = target.path().join(".claude/skills/dbt");
    fs::create_dir_all(&existing).unwrap();
    fs::write(existing.join("SKILL.md"), "old dbt\n").unwrap();
    fs::write(existing.join("removed-upstream.md"), "stale\n").unwrap();

    add_from(source.path(), target.path(), &["dbt"]).unwrap();

    assert_eq!(
        fs::read_to_string(existing.join("SKILL.md")).unwrap(),
        "dbt\n"
    );
    assert!(!existing.join("removed-upstream.md").exists());
}

/// REQ-008
#[test]
fn adding_an_unknown_category_fails_listing_available_ones_and_writes_nothing() {
    let source = fake_directives();
    let target = TempDir::new().unwrap();

    let error = add_from(source.path(), target.path(), &["core", "cobol"]).unwrap_err();

    assert_eq!(
        error.to_string(),
        "unknown category 'cobol' (available: core, dbt, rust)"
    );
    assert_eq!(fs::read_dir(target.path()).unwrap().count(), 0);
}

/// REQ-009
#[test]
fn adding_from_a_repo_clones_it_and_adds_the_categories() {
    let repo = fake_directives();
    git(repo.path(), &["init", "-q"]);
    git(repo.path(), &["add", "."]);
    git(
        repo.path(),
        &[
            "-c",
            "user.name=t",
            "-c",
            "user.email=t@t",
            "commit",
            "-q",
            "-m",
            "init",
        ],
    );
    let target = TempDir::new().unwrap();

    let repo_url = format!("file://{}", repo.path().display());
    add_from_repo(&repo_url, target.path(), &["core", "dbt"]).unwrap();

    let core = fs::read_to_string(target.path().join(".claude/core.md")).unwrap();
    assert_eq!(core, "# Core coding directives\n");
    let dbt = fs::read_to_string(target.path().join(".claude/skills/dbt/SKILL.md")).unwrap();
    assert_eq!(dbt, "dbt\n");
}
