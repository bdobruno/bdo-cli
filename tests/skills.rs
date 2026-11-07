use std::fs;

use bdo_cli::scripts::skills::list_from;
use tempfile::TempDir;

fn fake_directives() -> TempDir {
    let dir = TempDir::new().unwrap();
    let root = dir.path();
    fs::write(root.join("core.md"), "# Core coding directives\n").unwrap();
    fs::create_dir_all(root.join("skills/rust/rust-testing")).unwrap();
    fs::create_dir_all(root.join("skills/rust/rust-best-practices/references")).unwrap();
    fs::create_dir_all(root.join("skills/dbt")).unwrap();
    fs::write(root.join("skills/rust/rust-testing/SKILL.md"), "").unwrap();
    fs::write(root.join("skills/rust/rust-best-practices/SKILL.md"), "").unwrap();
    fs::write(root.join("skills/dbt/SKILL.md"), "").unwrap();
    dir
}

/// REQ-017
#[test]
fn listing_shows_each_category_with_its_skills_sorted() {
    let source = fake_directives();

    let listing = list_from(source.path()).unwrap();

    assert_eq!(
        listing,
        ["core", "dbt", "rust: rust-best-practices, rust-testing"]
    );
}
