pub mod add;
pub mod branch;
pub mod checkout;
pub mod create;
pub mod pr;
pub mod push;

use std::process::Command;

pub type ScriptResult = Result<(), Box<dyn std::error::Error>>;

fn run_command(program: &str, args: &[&str]) -> ScriptResult {
    let status = Command::new(program).args(args).status()?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("{program} exited with status {status}").into())
    }
}
