#![allow(deprecated)]

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_parse_issue_update() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("issue")
        .arg("update")
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn test_issue_update_requires_identifier() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("issue")
        .arg("update")
        .arg("--title")
        .arg("New title")
        .assert()
        .failure()
        .stderr(predicate::str::contains("IDENTIFIER"));
}

#[test]
fn test_issue_update_accepts_null_sentinel_for_clearable_fields() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("issue")
        .arg("update")
        .arg("ENG-123")
        .arg("--assignee")
        .arg("null")
        .arg("--project")
        .arg("null")
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn test_issue_update_rejects_out_of_range_priority() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("issue")
        .arg("update")
        .arg("ENG-123")
        .arg("--priority")
        .arg("9")
        .assert()
        .failure()
        .stderr(predicate::str::contains("priority"));
}

#[test]
fn test_issue_update_requires_at_least_one_patch_field() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("issue")
        .arg("update")
        .arg("ENG-123")
        .assert()
        .failure()
        .stderr(predicate::str::contains("at least one"));
}
