use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn run_with_defaults() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("batsay")
        .expect("binary exists")
        .assert()
        .success();
    Ok(())
}
