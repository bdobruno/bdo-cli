use super::{run_command, ScriptResult};

pub fn run(branch_name: &str) -> ScriptResult {
    if branch_name.is_empty() {
        return Err("branch name is required".into());
    }

    run_command("git", &["checkout", "-b", branch_name])?;
    run_command("git", &["push", "-u", "origin", branch_name])?;

    println!("Branch '{branch_name}' created and pushed to remote.");
    Ok(())
}
