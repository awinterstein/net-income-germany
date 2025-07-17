use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn calculate_for_current_year() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("net-income-germany-cmd")?;

    cmd.arg("--income").arg("80000");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(" 48173,"))
        .stdout(predicate::str::contains(" 80000,"));

    Ok(())
}

#[test]
fn calculate_reverse_for_current_year() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("net-income-germany-cmd")?;

    cmd.arg("--income").arg("60000").arg("--reverse");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(" 60000,"))
        .stdout(predicate::str::contains(" 103148,"));

    Ok(())
}

#[test]
fn error_on_unknown_year() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("net-income-germany-cmd")?;

    cmd.arg("--income").arg("80000");
    cmd.arg("--year").arg("2000");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No configuration available"));

    Ok(())
}

#[test]
fn error_on_too_large_inputs() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("net-income-germany-cmd")?;

    cmd.arg("--income")
        .arg((std::i32::MAX as u32 + 1).to_string());
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("too large"));

    Ok(())
}
