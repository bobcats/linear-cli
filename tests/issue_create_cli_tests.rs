#![allow(deprecated)]

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_parse_issue_create() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("issue")
        .arg("create")
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn test_issue_create_requires_team() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("issue")
        .arg("create")
        .arg("--title")
        .arg("Implement resolver layer")
        .assert()
        .failure()
        .stderr(predicate::str::contains("--team"));
}

#[test]
fn test_issue_create_requires_title() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("issue")
        .arg("create")
        .arg("--team")
        .arg("ENG")
        .assert()
        .failure()
        .stderr(predicate::str::contains("--title"));
}

#[test]
fn test_issue_create_accepts_rich_optional_fields() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("issue")
        .arg("create")
        .arg("--team")
        .arg("ENG")
        .arg("--title")
        .arg("Implement write support")
        .arg("--description")
        .arg("Track progress for issue write operations")
        .arg("--assignee")
        .arg("@me")
        .arg("--project")
        .arg("api-platform")
        .arg("--state")
        .arg("In Progress")
        .arg("--priority")
        .arg("2")
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn test_issue_create_rejects_out_of_range_priority() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("issue")
        .arg("create")
        .arg("--team")
        .arg("ENG")
        .arg("--title")
        .arg("Implement write support")
        .arg("--priority")
        .arg("9")
        .assert()
        .failure()
        .stderr(predicate::str::contains("priority"));
}
