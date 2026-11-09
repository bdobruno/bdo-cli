use std::error::Error;
use std::fmt;
use std::io::Write;
use std::process::{Command, Stdio};

use super::ScriptResult;

/// GitHub answered 404, which it also does when the account lacks permission.
#[derive(Debug)]
pub struct NotFound;

impl fmt::Display for NotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GitHub returned Not Found")
    }
}

impl Error for NotFound {}

/// The GitHub operations bdo needs, so tests can swap in a fake.
pub trait GitHub {
    /// Returns the login of the account gh is using.
    fn current_user(&self) -> Result<String, Box<dyn Error>>;
    /// Returns the repository's rulesets as the JSON array the API returns.
    fn list_rulesets(&self, repo: &str) -> Result<String, Box<dyn Error>>;
    fn create_ruleset(&self, repo: &str, ruleset: &str) -> ScriptResult;
    fn update_ruleset(&self, repo: &str, id: u64, ruleset: &str) -> ScriptResult;
}

/// The real GitHub, reached through the `gh` CLI.
pub struct GhCli;

impl GhCli {
    /// Returns the `owner/name` of the repository in the current directory.
    pub fn current_repo(&self) -> Result<String, Box<dyn Error>> {
        let repo = gh(
            &[
                "repo",
                "view",
                "--json",
                "nameWithOwner",
                "--jq",
                ".nameWithOwner",
            ],
            None,
        )?;
        Ok(repo.trim().to_string())
    }
}

impl GitHub for GhCli {
    fn current_user(&self) -> Result<String, Box<dyn Error>> {
        let user = gh(&["api", "user", "--jq", ".login"], None)?;
        Ok(user.trim().to_string())
    }

    fn list_rulesets(&self, repo: &str) -> Result<String, Box<dyn Error>> {
        gh(
            &["api", &format!("repos/{repo}/rulesets?per_page=100")],
            None,
        )
    }

    fn create_ruleset(&self, repo: &str, ruleset: &str) -> ScriptResult {
        let path = format!("repos/{repo}/rulesets");
        gh(
            &["api", "--method", "POST", &path, "--input", "-"],
            Some(ruleset),
        )?;
        Ok(())
    }

    fn update_ruleset(&self, repo: &str, id: u64, ruleset: &str) -> ScriptResult {
        let path = format!("repos/{repo}/rulesets/{id}");
        gh(
            &["api", "--method", "PUT", &path, "--input", "-"],
            Some(ruleset),
        )?;
        Ok(())
    }
}

/// Runs `gh` with `input` on stdin and returns what it printed.
fn gh(args: &[&str], input: Option<&str>) -> Result<String, Box<dyn Error>> {
    let mut child = Command::new("gh")
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(input.unwrap_or_default().as_bytes())?;
    }

    let output = child.wait_with_output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("(HTTP 404)") {
            return Err(Box::new(NotFound));
        }
        return Err(format!("gh failed: {}", stderr.trim()).into());
    }

    Ok(String::from_utf8(output.stdout)?)
}
