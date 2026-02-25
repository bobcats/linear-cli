#![allow(deprecated)]

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_parse_issue_comment_add() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("issue")
        .arg("comment")
        .arg("add")
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn test_issue_comment_add_requires_identifier() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("issue")
        .arg("comment")
        .arg("add")
        .arg("--body")
        .arg("Looks good")
        .assert()
        .failure()
        .stderr(predicate::str::contains("IDENTIFIER"));
}

#[test]
fn test_issue_comment_add_requires_body() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("issue")
        .arg("comment")
        .arg("add")
        .arg("ENG-123")
        .assert()
        .failure()
        .stderr(predicate::str::contains("--body"));
}
