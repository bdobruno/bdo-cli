pub mod add;
pub mod branch;
pub mod checkout;
pub mod create;
pub mod github;
pub mod pr;
pub mod protect;
pub mod push;
pub mod skills;

use std::process::Command;

pub type ScriptResult = Result<(), Box<dyn std::error::Error>>;

fn clone_directives(repo_url: &str) -> Result<tempfile::TempDir, Box<dyn std::error::Error>> {
    let clone = tempfile::tempdir()?;
    let clone_path = clone.path().to_str().ok_or("invalid temp dir path")?;
    run_command(
        "git",
        &["clone", "-q", "--depth", "1", repo_url, clone_path],
    )?;

    Ok(clone)
}

fn subdirectory_names(dir: &std::path::Path) -> Result<Vec<String>, std::io::Error> {
    let mut names = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            names.push(path.file_name().unwrap().to_string_lossy().into_owned());
        }
    }
    names.sort();

    Ok(names)
}

fn run_command(program: &str, args: &[&str]) -> ScriptResult {
    let status = Command::new(program).args(args).status()?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("{program} exited with status {status}").into())
    }
}
