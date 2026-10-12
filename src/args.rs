//! Translates the arguments the user typed back into Homebrew's words.
//! Only the subcommand and flags are touched; package names, repo names,
//! paths and anything after `--` pass through exactly as typed.

use crate::vocab::{self, Kind};
use std::collections::HashMap;
use std::ffi::OsString;

pub struct Translator {
    commands: HashMap<String, String>,
    flags: HashMap<String, String>,
}

impl Translator {
    pub fn new() -> Self {
        let mut commands = HashMap::new();
        let mut flags = HashMap::new();
        for x in vocab::entries() {
            match x.kind {
                Kind::Command => {
                    commands.insert(x.ours, x.homebrew);
                }
                Kind::Flag => {
                    flags.insert(x.ours, x.homebrew);
                }
                Kind::Word => {}
            }
        }
        Translator { commands, flags }
    }

    pub fn translate(&self, args: &[OsString]) -> Vec<OsString> {
        let typed_command = command(args).map(|c| self.commands.get(c).map_or(c, |h| h.as_str()));
        let mut out = Vec::with_capacity(args.len());
        let mut seen_command = false;
        let mut after_dashdash = false;
        for arg in args {
            let Some(s) = arg.to_str() else {
                out.push(arg.clone());
                continue;
            };
            if after_dashdash {
                out.push(arg.clone());
            } else if s == "--" {
                after_dashdash = true;
                out.push(arg.clone());
            } else if s.starts_with("--") {
                let name = s.split('=').next().unwrap_or(s);
                let keep = vocab::KEEP_FLAGS.iter().any(|(c, f)| Some(*c) == typed_command && *f == name);
                out.push(if keep { arg.clone() } else { self.flag(s).into() });
            } else if s.starts_with('-') {
                out.push(arg.clone());
            } else if !seen_command {
                seen_command = true;
                out.push(self.commands.get(s).map_or(s, |h| h.as_str()).into());
            } else {
                out.push(arg.clone());
            }
        }
        out
    }

    fn flag(&self, s: &str) -> String {
        let (name, value) = match s.split_once('=') {
            Some((n, v)) => (n, Some(v)),
            None => (s, None),
        };
        let name = self.flags.get(name).map_or(name, |h| h.as_str());
        match value {
            Some(v) => format!("{name}={v}"),
            None => name.to_string(),
        }
    }
}

/// The subcommand Homebrew will run, after translation.
pub fn command(args: &[OsString]) -> Option<&str> {
    args.iter()
        .filter_map(|a| a.to_str())
        .take_while(|a| *a != "--")
        .find(|a| !a.starts_with('-'))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(line: &str) -> String {
        let args: Vec<OsString> = line.split_whitespace().map(OsString::from).collect();
        Translator::new()
            .translate(&args)
            .iter()
            .map(|a| a.to_string_lossy().into_owned())
            .collect::<Vec<_>>()
            .join(" ")
    }

    #[test]
    fn swaps_commands_and_flags() {
        assert_eq!(run("unfollow user/repo"), "untap user/repo");
        assert_eq!(run("follow"), "tap");
        assert_eq!(run("follow-info --installed"), "tap-info --installed");
        assert_eq!(run("install --app zoom"), "install --cask zoom");
        assert_eq!(run("install --force-prebuilt wget"), "install --force-bottle wget");
        assert_eq!(run("list --package"), "list --formula");
        assert_eq!(run("--store"), "--cellar");
        assert_eq!(run("--store wget"), "--cellar wget");
        assert_eq!(run("cleanup --prebuilt-tag=arm64_sonoma x"), "cleanup --bottle-tag=arm64_sonoma x");
        assert_eq!(run("bundle --no-cleanup-follow"), "bundle --no-cleanup-tap");
        assert_eq!(run("bump-package-pr x"), "bump-formula-pr x");
    }

    #[test]
    fn leaves_names_alone() {
        // A package literally called "apps" or "follow" must not be renamed.
        assert_eq!(run("install apps follow"), "install apps follow");
        assert_eq!(run("info package"), "info package");
        assert_eq!(run("install wget"), "install wget");
        assert_eq!(run("sh -- --app"), "sh -- --app");
    }

    #[test]
    fn keeps_real_flags_that_use_our_words() {
        assert_eq!(run("bump-python-resources-pr --packages=x y"), "bump-python-resources-pr --packages=x y");
        assert_eq!(run("list --packages"), "list --formulae");
    }

    #[test]
    fn homebrew_words_still_work() {
        assert_eq!(run("tap user/repo"), "tap user/repo");
        assert_eq!(run("install --cask zoom"), "install --cask zoom");
    }
}
