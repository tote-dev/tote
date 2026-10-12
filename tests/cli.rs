//! Runs the real binary against a stand-in Homebrew (tests/fixtures/brew).

use std::process::{Command, Output};

fn tote(args: &[&str], env: &[(&str, &str)]) -> Output {
    let fake = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/brew");
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_tote"));
    cmd.args(args).env("TOTE_BACKEND", fake).env_remove("HOMEBREW_NO_AUTO_UPDATE");
    for (k, v) in env {
        cmd.env(k, v);
    }
    cmd.output().unwrap()
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

#[test]
fn translates_arguments_back() {
    // Raw mode, so the echoed arguments aren't translated again on the way out.
    let raw = [("TOTE_RAW", "1")];
    let out = tote(&["unfollow", "user/repo"], &raw);
    assert!(text(&out.stdout).contains("args: untap user/repo"));
    let out = tote(&["install", "--app", "zoom"], &raw);
    assert!(text(&out.stdout).contains("args: install --cask zoom"));
}

#[test]
fn rewrites_output() {
    let out = tote(&["install", "wget"], &[]);
    let stdout = text(&out.stdout);
    assert!(stdout.contains("==> Extracting wget--1.24.5.arm64_sonoma.bottle.tar.gz"), "{stdout}");
    assert!(stdout.contains("✓  /opt/homebrew/Cellar/wget/1.24.5: 92 files"), "{stdout}");
    assert!(stdout.contains("badge: ✓"), "{stdout}");
    let stderr = text(&out.stderr);
    assert!(stderr.contains("No available package with the name \"foo\". Did you mean to `tote follow` it?"), "{stderr}");
}

#[test]
fn raw_mode_changes_nothing() {
    let out = tote(&["install", "wget"], &[("TOTE_RAW", "1")]);
    assert!(text(&out.stdout).contains("==> Pouring"));
}

#[test]
fn passes_settings_on() {
    let out = tote(&["update"], &[("TOTE_NO_AUTO_UPDATE", "1")]);
    assert!(text(&out.stdout).contains("auto-update: 1"));
}

#[test]
fn keeps_exit_code() {
    let out = tote(&["install", "foo"], &[("FAKE_EXIT", "3")]);
    assert_eq!(out.status.code(), Some(3));
    let out = tote(&["shellenv"], &[("FAKE_EXIT", "4")]);
    assert_eq!(out.status.code(), Some(4));
}

#[test]
fn bundle_uses_bundlefile() {
    let dir = std::env::temp_dir().join(format!("tote-test-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_tote"))
        .args(["bundle"])
        .current_dir(&dir)
        .env("TOTE_BACKEND", concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/brew"))
        .env_remove("HOMEBREW_BUNDLE_FILE")
        .output()
        .unwrap();
    assert!(text(&out.stdout).contains("bundle-file: Bundlefile"));
    std::fs::remove_dir_all(&dir).unwrap();
}
