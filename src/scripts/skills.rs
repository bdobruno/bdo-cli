use std::error::Error;
use std::path::Path;

use super::{clone_directives, subdirectory_names, ScriptResult};
use crate::scripts::add::DIRECTIVES_REPO;

pub fn run() -> ScriptResult {
    let clone = clone_directives(DIRECTIVES_REPO)?;
    for line in list_from(clone.path())? {
        println!("{line}");
    }

    Ok(())
}

pub fn list_from(source: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    let skills_dir = source.join("skills");
    let mut listing = vec!["core".to_string()];
    for category in subdirectory_names(&skills_dir)? {
        let category_dir = skills_dir.join(&category);
        if category_dir.join("SKILL.md").exists() {
            listing.push(category);
        } else {
            let skills = subdirectory_names(&category_dir)?.join(", ");
            listing.push(format!("{category}: {skills}"));
        }
    }
    listing.sort();

    Ok(listing)
}
