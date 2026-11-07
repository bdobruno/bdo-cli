use super::{run_command, ScriptResult};

pub fn run() -> ScriptResult {
    run_command("gh", &["pr", "create", "--fill"])?;

    Ok(())
}
