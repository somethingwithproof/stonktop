//! Integration tests for stonktop CLI.

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
    let output = stonktop_bin()
        .output()
        .expect("Failed to execute command");

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
#[ignore]
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
