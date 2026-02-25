#![allow(deprecated)]

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_parse_issue_lifecycle_archive() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("issue")
        .arg("lifecycle")
        .arg("archive")
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn test_parse_issue_lifecycle_unarchive() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("issue")
        .arg("lifecycle")
        .arg("unarchive")
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn test_issue_lifecycle_archive_requires_identifier() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("issue")
        .arg("lifecycle")
        .arg("archive")
        .assert()
        .failure()
        .stderr(predicate::str::contains("IDENTIFIER"));
}
