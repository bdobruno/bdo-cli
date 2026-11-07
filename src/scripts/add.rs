use std::env;
use std::fs;
use std::path::Path;

use super::{run_command, ScriptResult};

pub const DIRECTIVES_REPO: &str = "https://github.com/bdobruno/ai-directives";

pub fn run(categories: &[&str]) -> ScriptResult {
    add_from_repo(DIRECTIVES_REPO, &env::current_dir()?, categories)
}

pub fn add_from_repo(repo_url: &str, target: &Path, categories: &[&str]) -> ScriptResult {
    let clone = tempfile::tempdir()?;
    let clone_path = clone.path().to_str().ok_or("invalid temp dir path")?;
    run_command(
        "git",
        &["clone", "-q", "--depth", "1", repo_url, clone_path],
    )?;

    add_from(clone.path(), target, categories)
}

const CORE_IMPORT: &str = "@.claude/core.md";
const AGENTS_POINTER: &str = "Follow the coding directives in `.claude/core.md`.";

pub fn add_from(source: &Path, target: &Path, categories: &[&str]) -> ScriptResult {
    validate_categories(categories, &available_categories(source)?)?;

    for category in categories {
        match *category {
            "core" => add_core(source, target)?,
            skill_category => add_skill_category(source, target, skill_category)?,
        }
    }

    Ok(())
}

fn available_categories(source: &Path) -> Result<Vec<String>, std::io::Error> {
    let mut categories = vec!["core".to_string()];
    for entry in fs::read_dir(source.join("skills"))? {
        let path = entry?.path();
        if path.is_dir() {
            categories.push(path.file_name().unwrap().to_string_lossy().into_owned());
        }
    }
    categories.sort();

    Ok(categories)
}

fn validate_categories(requested: &[&str], available: &[String]) -> Result<(), String> {
    match requested
        .iter()
        .find(|c| !available.iter().any(|a| a == *c))
    {
        Some(unknown) => Err(format!(
            "unknown category '{unknown}' (available: {})",
            available.join(", ")
        )),
        None => Ok(()),
    }
}

fn add_core(source: &Path, target: &Path) -> ScriptResult {
    let claude_dir = target.join(".claude");
    fs::create_dir_all(&claude_dir)?;
    fs::copy(source.join("core.md"), claude_dir.join("core.md"))?;
    ensure_line_in_file(&target.join("CLAUDE.md"), CORE_IMPORT)?;
    ensure_line_in_file(&target.join("AGENTS.md"), AGENTS_POINTER)?;

    Ok(())
}

fn add_skill_category(source: &Path, target: &Path, category: &str) -> ScriptResult {
    let skills_dir = target.join(".claude/skills");
    let category_dir = source.join("skills").join(category);
    if category_dir.join("SKILL.md").exists() {
        return replace_dir(&category_dir, &skills_dir.join(category));
    }

    for entry in fs::read_dir(category_dir)? {
        let skill = entry?.path();
        if skill.is_dir() {
            replace_dir(&skill, &skills_dir.join(skill.file_name().unwrap()))?;
        }
    }

    Ok(())
}

fn replace_dir(from: &Path, to: &Path) -> ScriptResult {
    if to.exists() {
        fs::remove_dir_all(to)?;
    }

    copy_dir(from, to)
}

fn copy_dir(from: &Path, to: &Path) -> ScriptResult {
    fs::create_dir_all(to)?;
    for entry in fs::read_dir(from)? {
        let path = entry?.path();
        let destination = to.join(path.file_name().unwrap());
        if path.is_dir() {
            copy_dir(&path, &destination)?;
        } else {
            fs::copy(&path, &destination)?;
        }
    }

    Ok(())
}

fn ensure_line_in_file(path: &Path, line: &str) -> ScriptResult {
    let existing = fs::read_to_string(path).unwrap_or_default();
    fs::write(path, with_line(&existing, line))?;

    Ok(())
}

fn with_line(existing: &str, line: &str) -> String {
    if existing.lines().any(|l| l.trim() == line) {
        return existing.to_string();
    }

    let body = existing.trim_end();
    if body.is_empty() {
        format!("{line}\n")
    } else {
        format!("{body}\n\n{line}\n")
    }
}
