use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;

use super::{clone_directives, subdirectory_names, ScriptResult};

pub const DIRECTIVES_REPO: &str = "https://github.com/bdobruno/ai-directives";

pub type Report = Result<Vec<String>, Box<dyn Error>>;

pub fn run(categories: &[&str], global: bool) -> ScriptResult {
    let report = if global {
        let home = env::var("HOME")?;
        let clone = clone_directives(DIRECTIVES_REPO)?;
        add_global_from(clone.path(), Path::new(&home), categories)?
    } else {
        let target = env::current_dir()?;
        if !is_git_repo(&target) {
            return Err("not a git repository — run bdo add inside a project".into());
        }
        add_from_repo(DIRECTIVES_REPO, &target, categories)?
    };

    for line in report {
        println!("{line}");
    }

    Ok(())
}

fn is_git_repo(dir: &Path) -> bool {
    Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .current_dir(dir)
        .output()
        .is_ok_and(|output| output.status.success())
}

pub fn add_from_repo(repo_url: &str, target: &Path, categories: &[&str]) -> Report {
    let clone = clone_directives(repo_url)?;
    add_from(clone.path(), target, categories)
}

pub fn add_global_from(source: &Path, home: &Path, categories: &[&str]) -> Report {
    if categories.iter().any(|category| *category != "core") {
        return Err("--global only supports core; skills are added per repository".into());
    }

    let claude_dir = home.join(".claude");
    fs::create_dir_all(&claude_dir)?;
    fs::copy(source.join("core.md"), claude_dir.join("core.md"))?;
    ensure_line_in_file(&claude_dir.join("CLAUDE.md"), GLOBAL_CORE_IMPORT)?;

    Ok(vec![installed(
        "core",
        "~/.claude/core.md, ~/.claude/CLAUDE.md",
    )])
}

const CORE_IMPORT: &str = "@.claude/core.md";
const GLOBAL_CORE_IMPORT: &str = "@~/.claude/core.md";
const AGENTS_POINTER: &str = "Follow the coding directives in `.claude/core.md`.";

pub fn add_from(source: &Path, target: &Path, categories: &[&str]) -> Report {
    validate_categories(categories, &available_categories(source)?)?;

    let mut report = Vec::new();
    for category in categories {
        match *category {
            "core" => report.push(add_core(source, target)?),
            skill_category => report.extend(add_skill_category(source, target, skill_category)?),
        }
    }

    Ok(report)
}

fn available_categories(source: &Path) -> Result<Vec<String>, std::io::Error> {
    let mut categories = subdirectory_names(&source.join("skills"))?;
    categories.push("core".to_string());
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

fn add_core(source: &Path, target: &Path) -> Result<String, Box<dyn Error>> {
    let claude_dir = target.join(".claude");
    fs::create_dir_all(&claude_dir)?;
    fs::copy(source.join("core.md"), claude_dir.join("core.md"))?;
    ensure_line_in_file(&target.join("CLAUDE.md"), CORE_IMPORT)?;
    ensure_line_in_file(&target.join("AGENTS.md"), AGENTS_POINTER)?;

    Ok(installed("core", ".claude/core.md, CLAUDE.md, AGENTS.md"))
}

fn add_skill_category(source: &Path, target: &Path, category: &str) -> Report {
    let category_dir = source.join("skills").join(category);
    let skills = if category_dir.join("SKILL.md").exists() {
        vec![category.to_string()]
    } else {
        subdirectory_names(&category_dir)?
    };

    let mut report = Vec::new();
    for skill in skills {
        let from = if skill == category {
            category_dir.clone()
        } else {
            category_dir.join(&skill)
        };
        let relative = format!(".claude/skills/{skill}");
        replace_dir(&from, &target.join(&relative))?;
        report.push(installed(&skill, &relative));
    }

    Ok(report)
}

fn installed(name: &str, location: &str) -> String {
    format!("✓ {name} → {location}")
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
