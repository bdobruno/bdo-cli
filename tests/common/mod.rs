// Each test binary uses a different subset of these helpers.
#![allow(dead_code)]

use std::cell::RefCell;
use std::error::Error;

use bdo_cli::scripts::github::GitHub;
use bdo_cli::scripts::ScriptResult;
use serde_json::{json, Value};

#[derive(Clone, Debug, PartialEq)]
pub struct Ruleset {
    pub repo: String,
    pub id: u64,
    pub body: String,
}

/// In-memory GitHub that stores rulesets the way the API does.
#[derive(Default)]
pub struct FakeGitHub {
    rulesets: RefCell<Vec<Ruleset>>,
}

impl FakeGitHub {
    pub fn with_rulesets(rulesets: Vec<Ruleset>) -> Self {
        Self {
            rulesets: RefCell::new(rulesets),
        }
    }

    pub fn rulesets(&self) -> Vec<Ruleset> {
        self.rulesets.borrow().clone()
    }
}

fn name_of(body: &str) -> String {
    let value: Value = serde_json::from_str(body).unwrap();
    value["name"].as_str().unwrap().to_string()
}

impl GitHub for FakeGitHub {
    fn list_rulesets(&self, repo: &str) -> Result<String, Box<dyn Error>> {
        let listed: Vec<Value> = self
            .rulesets
            .borrow()
            .iter()
            .filter(|ruleset| ruleset.repo == repo)
            .map(|ruleset| json!({ "id": ruleset.id, "name": name_of(&ruleset.body) }))
            .collect();
        Ok(Value::from(listed).to_string())
    }

    fn create_ruleset(&self, repo: &str, ruleset: &str) -> ScriptResult {
        let mut rulesets = self.rulesets.borrow_mut();
        let id = rulesets.len() as u64 + 1;
        rulesets.push(Ruleset {
            repo: repo.to_string(),
            id,
            body: ruleset.to_string(),
        });
        Ok(())
    }

    fn update_ruleset(&self, repo: &str, id: u64, ruleset: &str) -> ScriptResult {
        let mut rulesets = self.rulesets.borrow_mut();
        let existing = rulesets
            .iter_mut()
            .find(|existing| existing.repo == repo && existing.id == id)
            .ok_or("ruleset not found")?;
        existing.body = ruleset.to_string();
        Ok(())
    }
}
