//! Runs Homebrew with translated arguments and rewrites what it prints.

use crate::brand;
use crate::filter::Filter;
use std::env;
use std::ffi::OsString;
use std::io::{self, IsTerminal, Read, Write};
use std::os::unix::process::{CommandExt, ExitStatusExt};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;

/// Settings read by this tool itself; every other `<PREFIX>_X` goes to Homebrew.
const OWN_SETTINGS: &[&str] = &["RAW", "HIDE_PATHS", "BACKEND"];

/// Commands whose output is read by scripts or shells, or that need the
/// terminal to themselves. They run untouched.
const UNFILTERED: &[&str] = &[
    "sh", "irb", "debugger", "ruby", "edit", "shellenv", "--env", "--prefix", "--cellar",
    "--caskroom", "--repository", "--repo", "--cache", "completions",
];

pub fn setting(name: &str) -> Option<String> {
    env::var(format!("{}_{}", brand::ENV_PREFIX, name)).ok()
}

fn flag_on(name: &str) -> bool {
    setting(name).is_some_and(|v| !v.is_empty() && v != "0")
}

pub fn find_homebrew() -> Option<PathBuf> {
    if let Some(p) = setting("BACKEND") {
        return Some(PathBuf::from(p));
    }
    let on_path = env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths)
            .map(|dir| dir.join("brew"))
            .find(|p| is_executable(p))
    });
    on_path.or_else(|| {
        ["/opt/homebrew/bin/brew", "/usr/local/bin/brew", "/home/linuxbrew/.linuxbrew/bin/brew"]
            .iter()
            .map(PathBuf::from)
            .find(|p| is_executable(p))
    })
}

fn is_executable(p: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    p.metadata().is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
}

fn prepare(homebrew: &Path, args: &[OsString]) -> Command {
    let mut cmd = Command::new(homebrew);
    cmd.args(args);

    // TOTE_X=1 is passed on as HOMEBREW_X=1 unless HOMEBREW_X is already set.
    let prefix = format!("{}_", brand::ENV_PREFIX);
    for (key, value) in env::vars_os() {
        let Some(rest) = key.to_str().and_then(|k| k.strip_prefix(&prefix)) else {
            continue;
        };
        if rest.is_empty() || OWN_SETTINGS.contains(&rest) {
            continue;
        }
        let target = format!("HOMEBREW_{rest}");
        if env::var_os(&target).is_none() {
            cmd.env(target, value);
        }
    }
    if env::var_os("HOMEBREW_INSTALL_BADGE").is_none() {
        cmd.env("HOMEBREW_INSTALL_BADGE", "\u{2713}");
    }
    if crate::args::command(args) == Some("bundle") {
        use_bundlefile(&mut cmd, args);
    }
    cmd
}

/// `bundle` reads and writes a Bundlefile, unless a Brewfile is already there
/// or the user picked a file.
fn use_bundlefile(cmd: &mut Command, args: &[OsString]) {
    let chose = args.iter().filter_map(|a| a.to_str()).any(|a| {
        a == "--file" || a.starts_with("--file=") || a == "--global" || a == "-g"
    });
    if chose || env::var_os("HOMEBREW_BUNDLE_FILE").is_some() {
        return;
    }
    if Path::new("Bundlefile").exists() || !Path::new("Brewfile").exists() {
        cmd.env("HOMEBREW_BUNDLE_FILE", "Bundlefile");
    }
}

fn should_filter(args: &[OsString]) -> bool {
    if flag_on("RAW") {
        return false;
    }
    let strs: Vec<&str> = args.iter().filter_map(|a| a.to_str()).collect();
    // JSON is read by programs, so its keys must stay exactly as Homebrew wrote them.
    if strs.iter().any(|a| a.starts_with("--json")) {
        return false;
    }
    match strs.iter().find(|a| !a.starts_with('-') || UNFILTERED.contains(a)) {
        Some(first) => !UNFILTERED.contains(first),
        None => true,
    }
}

/// Runs Homebrew and returns the exit code to finish with.
pub fn run(homebrew: &Path, args: &[OsString]) -> i32 {
    let mut cmd = prepare(homebrew, args);

    if !should_filter(args) {
        // Nothing to rewrite: become Homebrew, so behaviour is identical.
        let err = cmd.exec();
        eprintln!("{}: could not run {}: {err}", brand::NAME, homebrew.display());
        return 127;
    }

    // Output goes through a pipe, so tell Homebrew to keep its colours when
    // the user is looking at a terminal.
    if io::stdout().is_terminal()
        && env::var_os("HOMEBREW_COLOR").is_none()
        && env::var_os("HOMEBREW_NO_COLOR").is_none()
        && env::var_os("NO_COLOR").is_none()
    {
        cmd.env("HOMEBREW_COLOR", "1");
    }
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

    let mut child = match cmd.spawn() {
        Ok(c) => c,
        Err(err) => {
            eprintln!("{}: could not run {}: {err}", brand::NAME, homebrew.display());
            return 127;
        }
    };
    // Ctrl-C reaches Homebrew directly; we stay alive to pass on its last words.
    unsafe {
        libc::signal(libc::SIGINT, libc::SIG_IGN);
        libc::signal(libc::SIGQUIT, libc::SIG_IGN);
    }

    let hide_paths = flag_on("HIDE_PATHS");
    let out = child.stdout.take().unwrap();
    let err = child.stderr.take().unwrap();
    let t1 = thread::spawn(move || pump(out, io::stdout(), hide_paths));
    let t2 = thread::spawn(move || pump(err, io::stderr(), hide_paths));
    let _ = t1.join();
    let _ = t2.join();

    match child.wait() {
        Ok(status) => status
            .code()
            .unwrap_or_else(|| 128 + status.signal().unwrap_or(1)),
        Err(_) => 1,
    }
}

/// Copies `from` to `to`, rewriting as it goes. Text is passed on at every line
/// break (`\n` or `\r`, so progress bars still move) and whenever Homebrew
/// pauses after something that isn't half a word (so prompts appear).
fn pump(mut from: impl Read, mut to: impl Write, hide_paths: bool) {
    let filter = Filter::new(hide_paths);
    let mut buf = [0u8; 8192];
    let mut pending: Vec<u8> = Vec::new();
    let mut broken = false;
    loop {
        let n = match from.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(_) => break,
        };
        pending.extend_from_slice(&buf[..n]);
        let cut = match pending.iter().rposition(|b| *b == b'\n' || *b == b'\r') {
            Some(p) => p + 1,
            None if pending.last().is_some_and(|b| !b.is_ascii_alphanumeric()) => pending.len(),
            None if pending.len() > 64 * 1024 => pending.len(),
            None => 0,
        };
        if cut > 0 && !broken {
            let chunk: Vec<u8> = pending.drain(..cut).collect();
            broken = write_all(&mut to, &filter.rewrite(&chunk)).is_err();
        } else if broken {
            // The reader went away (e.g. `| head`); keep draining so Homebrew finishes.
            pending.clear();
        }
    }
    if !pending.is_empty() && !broken {
        let _ = write_all(&mut to, &filter.rewrite(&pending));
    }
}

fn write_all(to: &mut impl Write, bytes: &[u8]) -> io::Result<()> {
    to.write_all(bytes)?;
    to.flush()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(s: &str) -> Vec<OsString> {
        s.split_whitespace().map(OsString::from).collect()
    }

    #[test]
    fn filters_normal_commands_only() {
        assert!(should_filter(&args("install wget")));
        assert!(should_filter(&args("info --cask zoom")));
        assert!(should_filter(&args("")));
        assert!(!should_filter(&args("info --json=v2 wget")));
        assert!(!should_filter(&args("shellenv")));
        assert!(!should_filter(&args("--prefix wget")));
        assert!(!should_filter(&args("-v edit wget")));
    }
}
