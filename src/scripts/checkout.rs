use std::process::Command;

use super::{run_command, ScriptResult};

pub fn run() -> ScriptResult {
    let output = Command::new("git")
        .args(["branch", "--show-current"])
        .output()?;

    if !output.status.success() {
        return Err(format!(
            "git branch --show-current exited with status {}",
            output.status
        )
        .into());
    }

    let branch_name = String::from_utf8(output.stdout)?.trim().to_owned();

    if branch_name == "main" || branch_name == "master" {
        println!("You are already on the main branch.");
        return Ok(());
    }

    run_command("git", &["switch", "main"])?;
    run_command("git", &["branch", "-d", &branch_name])?;
    run_command("git", &["fetch", "origin"])?;
    run_command("git", &["pull", "origin", "main"])?;
    run_command("git", &["reset", "--hard", "origin/main"])?;

    println!("Branch {branch_name} deleted and main is now up-to-date!");
    Ok(())
}
