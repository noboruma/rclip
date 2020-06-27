use assert_cmd::Command;

#[test]
fn check_default() {
    let mut cmd = Command::cargo_bin("rclip").unwrap();
    cmd.assert().failure();
}

#[test]
fn check_helper() {
    let mut cmd = Command::cargo_bin("rclip").unwrap();

    cmd.args(&["-h"]).assert().success();
    cmd.args(&["--help"]).assert().success();
}

#[test]
fn check_network_error() {
    let mut cmd = Command::cargo_bin("rclip").unwrap();
    cmd.args(&["paste"]).env("RCLIP_URL", "https://google.stuff").env("HOME", "/tmp").assert().failure();
}

#[test]
fn check_directory_error() {
    let mut cmd = Command::cargo_bin("rclip").unwrap();
    cmd.args(&["paste"]).env("RCLIP_URL", "https://google.stuff").env("HOME", "").assert().failure();
}

#[test]
fn check_illformed_url() {
    let mut cmd = Command::cargo_bin("rclip").unwrap();
    cmd.args(&["paste"]).env("RCLIP_URL", "https://google.").env("HOME", "/tmp").assert().failure();
}

#[test]
fn check_no_url() {
    let mut cmd = Command::cargo_bin("rclip").unwrap();
    cmd.args(&["paste"]).env("HOME", "/tmp").assert().failure();
}

#[test]
fn check_many_args() {
    let mut cmd = Command::cargo_bin("rclip").unwrap();
    cmd.args(&["paste", "paste", "paste"]).env("RCLIP_URL", "https://google.").env("HOME", "/tmp").assert().failure();
}

#[test]
fn check_pull_no_backend() {
    let mut cmd = Command::cargo_bin("rclip").unwrap();
    cmd.args(&["paste"]).env("RCLIP_URL", "https://google.co").env("HOME", "/tmp").assert().failure();
}

#[test]
fn check_push_no_backend() {
    let mut cmd = Command::cargo_bin("rclip").unwrap();
    cmd.args(&["copy", "copy"]).env("RCLIP_URL", "https://google.co").env("HOME", "/tmp").assert().success();
}

#[test]
fn check_open_no_backend() {
    let mut cmd = Command::cargo_bin("rclip").unwrap();
    cmd.args(&["open"]).env("RCLIP_URL", "https://google.co").env("HOME", "/tmp").assert().failure();
}

#[test]
fn check_link_no_backend() {
    let mut cmd = Command::cargo_bin("rclip").unwrap();
    cmd.args(&["link", "link"]).env("RCLIP_URL", "https://google.co").env("HOME", "/tmp").assert().failure();
}
