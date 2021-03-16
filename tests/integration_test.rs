use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn run_with_defaults() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("batsay")
        .expect("binary exists")
        .assert()
        .success()
        .stdout(predicate::str::contains("Bat Noises!"));
    Ok(())
}

#[test]
fn run_with_valid_text_message() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("batsay")
        .expect("binary exists")
        .args(&["My Awesome Message"])
        .assert()
        .success()
        .stdout(predicate::str::contains("My Awesome Message"));

    Ok(())
}

#[test]
fn run_with_invalid_file() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("batsay")
        .expect("binary exists")
        .args(&["-f", "no/such/file.txt"])
        .assert()
        .failure();

    Ok(())
}

#[test]
fn run_with_valid_file() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("batsay")
        .expect("binary exists")
        .args(&["-f", "./ascii_files/gollum.txt"])
        .assert()
        .success();

    Ok(())
}

#[test]
fn run_with_dead_flag() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("batsay")
        .expect("binary exists")
        .args(&["-d"])
        .assert()
        .success()
        .stdout(predicate::str::contains(" x x "));

    Ok(())
}

#[test]
fn run_with_uwu_flag() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("batsay")
        .expect("binary exists")
        .args(&["--uwu"])
        .assert()
        .success()
        .stdout(predicate::str::contains(" u u "));

    Ok(())
}
