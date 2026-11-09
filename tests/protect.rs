mod common;

use bdo_cli::scripts::protect::{self, RULESET};
use common::{FakeGitHub, Ruleset};

/// REQ-019
#[test]
fn protecting_an_unprotected_repo_creates_the_ruleset() {
    let github = FakeGitHub::default();

    protect::apply(&github, "me/my-repo").unwrap();

    assert_eq!(
        github.rulesets(),
        [Ruleset {
            repo: "me/my-repo".to_string(),
            id: 1,
            body: RULESET.to_string(),
        }]
    );
}

/// REQ-020
#[test]
fn protecting_an_already_protected_repo_updates_the_ruleset_in_place() {
    let other = Ruleset {
        repo: "me/my-repo".to_string(),
        id: 2,
        body: r#"{"name": "release-tags"}"#.to_string(),
    };
    let github = FakeGitHub::with_rulesets(vec![
        Ruleset {
            repo: "me/my-repo".to_string(),
            id: 1,
            body: r#"{"name": "protect-main", "enforcement": "disabled"}"#.to_string(),
        },
        other.clone(),
    ]);

    protect::apply(&github, "me/my-repo").unwrap();

    assert_eq!(
        github.rulesets(),
        [
            Ruleset {
                repo: "me/my-repo".to_string(),
                id: 1,
                body: RULESET.to_string(),
            },
            other,
        ]
    );
}
