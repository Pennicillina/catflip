use std::process::Command;
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn run_with_defaults() {
    Command::cargo_bin("catflip")
        .expect("error: binary not found")
        .assert()
        .success()
        .stdout(predicate::str::contains("HELLO"));
}

#[test]
fn run_with_message() {
    let mut cmd = Command::cargo_bin("catflip")
        .expect("error: binary not found");
    cmd.arg("5508");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("BOSS"));
}