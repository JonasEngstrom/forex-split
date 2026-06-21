use std::env;
use std::io::Write;
use std::process::{Command, Stdio};

fn binary_path() -> String {
    env::var("CARGO_BIN_EXE_forex-split")
        .or_else(|_| env::var("CARGO_BIN_EXE_forex_split"))
        .expect("binary path not set by cargo test")
}

fn run_binary(args: &[&str], input: &str) -> std::process::Output {
    let mut child = Command::new(binary_path())
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn binary");

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(input.as_bytes())
            .expect("failed to write test input");
    }

    child.wait_with_output().expect("failed to read output")
}

fn stdout_text(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr_text(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

#[test]
fn cli_handles_all_arguments_and_named_categories() {
    let output = run_binary(
        &["299.28", "26.84"],
        "Alcohol 6.9\nFood 17.5\n\n",
    );

    assert!(output.status.success());

    let stdout = stdout_text(&output);
    assert!(stdout.contains("Category"));
    assert!(stdout.contains("Foreign subtotal"));
    assert!(stdout.contains("Domestic subtotal"));
    assert!(stdout.contains("Alcohol"));
    assert!(stdout.contains("Food"));
    assert!(stdout.contains("76.94"));
    assert!(stdout.contains("195.13"));
    assert!(stdout.contains("Other"));
    assert!(stdout.contains("27.21"));
}

#[test]
fn cli_prompts_for_missing_foreign_total() {
    let output = run_binary(&["299.28"], "26.84\n\n");

    assert!(output.status.success());

    let stdout = stdout_text(&output);
    assert!(stdout.contains("Please enter foreign total:"));
    assert!(stdout.contains("Other"));
    assert!(stdout.contains("26.84"));
    assert!(stdout.contains("299.28"));
}

#[test]
fn cli_prompts_for_both_missing_totals() {
    let output = run_binary(&[], "299.28\n26.84\n\n");

    assert!(output.status.success());

    let stdout = stdout_text(&output);
    assert!(stdout.contains("Please enter domestic total:"));
    assert!(stdout.contains("Please enter foreign total:"));
    assert!(stdout.contains("Other"));
}

#[test]
fn cli_handles_unnamed_categories_and_other() {
    let output = run_binary(&["299.28", "26.84"], "6.9\n17.5\n\n");

    assert!(output.status.success());

    let stdout = stdout_text(&output);
    assert!(stdout.contains("Unnamed category 1"));
    assert!(stdout.contains("Unnamed category 2"));
    assert!(stdout.contains("Other"));
}

#[test]
fn cli_accumulates_repeated_named_categories() {
    let output = run_binary(&["299.28", "26.84"], "Food 9.9\nFood 7.59\n\n");

    assert!(output.status.success());

    let stdout = stdout_text(&output);
    assert!(stdout.contains("Food"));
    assert!(stdout.contains("17.49"));
    assert!(stdout.contains("Other"));
    assert!(stdout.contains("9.35"));
}

#[test]
fn cli_reports_invalid_subtotal_and_recovers() {
    let output = run_binary(&["299.28", "26.84"], "Alcohol foo\nAlcohol 6.9\n\n");

    assert!(output.status.success());

    let stderr = stderr_text(&output);
    let stdout = stdout_text(&output);
    assert!(stderr.contains("Please enter the subtotal as the last element on the line"));
    assert!(stdout.contains("Alcohol"));
    assert!(stdout.contains("Other"));
}
