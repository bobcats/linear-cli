#![allow(deprecated)]

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_parse_auth_login() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("auth")
        .arg("login")
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn test_parse_auth_login_with_token_flag() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("auth")
        .arg("login")
        .arg("--with-token")
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn test_parse_auth_status() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("auth")
        .arg("status")
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn test_parse_auth_logout() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("auth")
        .arg("logout")
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn test_parse_auth_token() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("auth")
        .arg("token")
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn test_invalid_auth_subcommand() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("auth")
        .arg("invalid")
        .assert()
        .failure()
        .stderr(predicate::str::contains("unrecognized subcommand"));
}

#[test]
fn test_no_subcommand_shows_help() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("auth")
        .assert()
        .failure()
        .stderr(predicate::str::contains("auth <COMMAND>"));
}

#[test]
fn test_parse_issue_view() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("issue")
        .arg("view")
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn test_issue_view_requires_identifier() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("issue")
        .arg("view")
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_parse_issue_list() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("issue")
        .arg("list")
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn test_parse_issue_list_with_limit() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("issue")
        .arg("list")
        .arg("--limit")
        .arg("10")
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn test_parse_issue_list_with_assignee() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("issue")
        .arg("list")
        .arg("--assignee")
        .arg("@me")
        .arg("--help")
        .assert()
        .success();
}

// ── Discovery command parsing tests ──

#[test]
fn test_parse_state_list() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("state")
        .arg("list")
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn test_parse_state_list_with_team_and_limit() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("state")
        .arg("list")
        .arg("--team")
        .arg("ENG")
        .arg("--limit")
        .arg("10")
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn test_parse_label_list() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("label")
        .arg("list")
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn test_parse_label_list_with_team() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("label")
        .arg("list")
        .arg("--team")
        .arg("DESIGN")
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn test_parse_user_list() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("user")
        .arg("list")
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn test_parse_user_list_with_limit() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("user")
        .arg("list")
        .arg("--limit")
        .arg("25")
        .arg("--help")
        .assert()
        .success();
}

// ── Issue search command parsing tests ──

#[test]
fn test_parse_issue_search() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("issue")
        .arg("search")
        .arg("token refresh")
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn test_parse_issue_search_with_team_and_limit() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("issue")
        .arg("search")
        .arg("authentication")
        .arg("--team")
        .arg("ENG")
        .arg("--limit")
        .arg("10")
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn test_issue_search_requires_term() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("issue")
        .arg("search")
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

// ── Delete command parsing tests ──

#[test]
fn test_parse_issue_delete() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("issue")
        .arg("delete")
        .arg("ENG-123")
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn test_parse_issue_delete_requires_identifier() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("issue")
        .arg("delete")
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_parse_comment_delete() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("issue")
        .arg("comment")
        .arg("delete")
        .arg("comment-uuid-123")
        .arg("--help")
        .assert()
        .success();
}

// ── Semantic search command parsing tests ──

#[test]
fn test_parse_search() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("search")
        .arg("authentication flow")
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn test_parse_search_with_type_filter() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("search")
        .arg("bidding")
        .arg("--type")
        .arg("issue")
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn test_search_requires_query() {
    Command::cargo_bin("linear-cli")
        .unwrap()
        .arg("search")
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}
