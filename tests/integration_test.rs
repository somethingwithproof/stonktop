//! Integration tests for stonktop CLI.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::process::Command;

/// Get the path to the stonktop binary.
fn stonktop_bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_stonktop"))
}

#[test]
fn test_help_flag() {
    let output = stonktop_bin()
        .arg("--help")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("stonktop"));
    assert!(stdout.contains("terminal UI"));
    assert!(stdout.contains("--symbols"));
    assert!(stdout.contains("--delay"));
}

#[test]
fn test_version_flag() {
    let output = stonktop_bin()
        .arg("--version")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("stonktop"));
    // Version should match semver pattern
    assert!(stdout.contains("0.") || stdout.contains("1."));
}

#[test]
fn test_no_symbols_error() {
    let output = stonktop_bin().output().expect("Failed to execute command");

    // Should exit with error when no symbols provided
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("No symbols to watch") || stderr.contains("symbols"));
}

#[test]
fn test_invalid_delay() {
    let output = stonktop_bin()
        .args(["-s", "AAPL", "-d", "invalid"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
}

/// Test batch mode with network access.
/// This test is ignored by default as it requires network access.
/// Run with: cargo test -- --ignored
#[test]
#[ignore = "hits the live network"]
fn test_batch_mode_with_network() {
    let child = stonktop_bin()
        .args(["-s", "AAPL", "-b", "-n", "1", "--timeout", "5"])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to start command");

    // Wait with timeout
    let output = child
        .wait_with_output()
        .expect("Failed to wait for command");

    // In batch mode with 1 iteration, should complete
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("STONKTOP") || stdout.contains("AAPL"));
    }
    // Network failure is acceptable in CI
}

#[test]
fn test_sort_options() {
    // Test that sort option is accepted
    let output = stonktop_bin()
        .args(["--help"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--sort"));
    assert!(stdout.contains("symbol"));
    assert!(stdout.contains("price"));
    assert!(stdout.contains("change"));
}

#[test]
fn test_config_path_option() {
    let output = stonktop_bin()
        .args(["--help"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--config"));
    assert!(stdout.contains("-c"));
}

#[test]
fn test_holdings_flag() {
    let output = stonktop_bin()
        .args(["--help"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--holdings"));
    assert!(stdout.contains("-H"));
}

#[test]
fn test_secure_mode_flag() {
    let output = stonktop_bin()
        .args(["--help"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--secure"));
    assert!(stdout.contains("-S"));
}

#[test]
fn test_env_vars_documented() {
    let output = stonktop_bin()
        .args(["--help"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("STONKTOP_SYMBOLS") || stdout.contains("env"));
}

/// Live E2E / integration test running the *packaged* app inside Docker.
///
/// This exercises the release artifact (the binary as it would be distributed)
/// in a containerized "live" environment (reproducible base OS, no host pollution).
///
/// Requires Docker daemon + the image built locally with the provided Dockerfile:
///   docker build -t stonktop:test .
///
/// Run locally: cargo test --test integration_test test_docker_live_e2e -- --ignored
///
/// In CI this is driven by the dedicated "docker" job which builds the image once
/// then invokes this test (with --ignored).
#[test]
#[ignore = "requires a locally built stonktop:test image"]
fn test_docker_live_e2e() {
    // Graceful skip if no docker in the environment (common in some dev setups).
    let docker_available = Command::new("docker")
        .arg("info")
        .output()
        .is_ok_and(|output| output.status.success());
    if !docker_available {
        eprintln!("skipping test_docker_live_e2e: 'docker' command not available (requires Docker daemon + `docker build -t stonktop:test .`)");
        return;
    }

    // Use a fixed tag that the CI docker job and local instructions use.
    let image = "stonktop:test";

    // Run the container in batch mode for 1 iteration against a common symbol.
    // --rm for cleanup, no tty (batch mode is fine), short timeout to keep test fast.
    let output = Command::new("docker")
        .args([
            "run",
            "--rm",
            image,
            "-s",
            "AAPL",
            "-b",
            "-n",
            "1",
            "--timeout",
            "8",
        ])
        .output()
        .expect("Failed to run docker. Did you `docker build -t stonktop:test .` first?");

    // The containerized live request must succeed and produce recognizable output.
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(output.status.success(), "docker live e2e failed: {stderr}");
    assert!(
        stdout.contains("AAPL") || stdout.contains("STONKTOP") || stdout.contains("price"),
        "expected recognizable batch output from live docker run, got: {stdout}"
    );
}
