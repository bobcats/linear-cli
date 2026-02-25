#![allow(deprecated)]

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_parse_issue_relation_link() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("issue")
        .arg("relation")
        .arg("link")
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn test_parse_issue_relation_block() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("issue")
        .arg("relation")
        .arg("block")
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn test_parse_issue_relation_duplicate() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("issue")
        .arg("relation")
        .arg("duplicate")
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn test_issue_relation_link_requires_both_issue_identifiers() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("issue")
        .arg("relation")
        .arg("link")
        .arg("ENG-123")
        .assert()
        .failure()
        .stderr(predicate::str::contains("RELATED"));
}
