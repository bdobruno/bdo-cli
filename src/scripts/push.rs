use super::{run_command, ScriptResult};

pub fn run() -> ScriptResult {
    run_command("git", &["add", "."])?;
    run_command("git", &["commit"])?;
    run_command("git", &["push"])?;

    Ok(())
}
