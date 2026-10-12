//! The word list from VOCABULARY.md. Each Homebrew word has exactly one
//! replacement, used in both directions:
//! - commands and flags the user types are translated back to Homebrew's words;
//! - Homebrew's output is translated to ours.
//!
//! Command and flag names are not listed by hand: they are built from the real
//! Homebrew names below by swapping each word in them, so `--no-cleanup-tap`
//! always becomes `--no-cleanup-follow`.

use crate::brand;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    /// A subcommand, translated only in command position (`tote follow x`).
    Command,
    /// A flag, translated anywhere before `--` (`--app`).
    Flag,
    /// Only appears in output.
    Word,
}

pub struct Entry {
    pub homebrew: String,
    pub ours: String,
    pub kind: Kind,
}

/// One replacement per Homebrew word. These are also the parts that
/// command and flag names are built from. All lowercase.
const WORDS: &[(&str, &str)] = &[
    ("tap", "follow"),
    ("taps", "follows"),
    ("untap", "unfollow"),
    ("cask", "app"),
    ("casks", "apps"),
    ("caskroom", "approom"),
    ("formula", "package"),
    ("formulae", "packages"),
    ("bottle", "prebuilt"),
    ("bottles", "prebuilts"),
    ("bottled", "prebuilt"),
    ("unbottled", "unprebuilt"),
    ("keg", "folder"),
    ("kegs", "folders"),
    ("cellar", "store"),
    ("brewfile", "bundlefile"),
    ("poured", "extracted"),
    ("unbrewed", "unbuilt"),
];

/// Forms that only appear in Homebrew's output.
const OUTPUT_WORDS: &[(&str, &str)] = &[
    ("tapped", "followed"),
    ("tapping", "following"),
    ("untapped", "unfollowed"),
    ("untapping", "unfollowing"),
    ("formulas", "packages"),
    ("keg-only", "folder-only"),
    ("pour", "extract"),
    ("pours", "extracts"),
    ("pouring", "extracting"),
    ("brewing", "building"),
    ("brewed", "built"),
];

/// Every Homebrew command containing one of the words above, taken from
/// Homebrew's own `cmd/` and `dev-cmd/` folders.
const HOMEBREW_COMMANDS: &[&str] = &[
    "as-brew-user", "bottle", "bump-cask-pr", "bump-formula-pr", "bump-unversioned-casks",
    "cask-ci", "casks", "dispatch-build-bottle", "formula", "formula-analytics",
    "formula-python-resources", "formulae", "generate-bottle-ci-matrix", "generate-cask-api",
    "generate-cask-ci-matrix", "generate-cask-token", "generate-formula-api", "tap", "tap-info",
    "tap-new", "unbottled", "untap", "which-formula",
];

/// Every Homebrew flag containing one of the words above (without `--`).
/// `--cellar`, `--caskroom` and `--taps` are commands Homebrew spells as flags.
const HOMEBREW_FLAGS: &[&str] = &[
    "added-formulae", "all-core-formulae-json", "bottle-json-dir", "bottle-tag",
    "brew-command-run", "brew-command-run-options", "brew-test-bot-test", "brewfile",
    "build-bottle", "cask", "cask-install", "caskroom", "casks", "cellar", "deleted-formulae",
    "force-bottle", "force-core-tap", "formula", "formula-list", "formulae",
    "formulae-dependents-shard", "homebrew-devcmdrun-developer", "homebrew-env-config",
    "homebrew-os-arch-ci", "homebrew-prefixes", "homebrew-versions", "no-cask", "no-casks",
    "no-cleanup-brew", "no-cleanup-cask", "no-cleanup-tap", "no-dump-brew", "no-dump-cask",
    "no-dump-tap", "no-formula", "no-formulae", "no-tap", "no-taps", "only-bottles-fetch",
    "only-formulae", "only-formulae-dependents", "only-formulae-detect", "only-tap-syntax",
    "poured-from-bottle", "remove-bottle-block", "removed-formulae-file", "retain-bottle-dir",
    "skipped-or-failed-formulae", "tap", "taps", "test-default-formula", "tested-formulae",
    "testing-formulae", "unbrewed", "upgrade-formula", "upgrade-formulae",
];

/// Real Homebrew flags that already use one of our words, so the user's
/// spelling must not be translated for that command.
pub const KEEP_FLAGS: &[(&str, &str)] = &[("bump-python-resources-pr", "--packages")];

/// Emoji replaced in output.
pub const EMOJI: &[(&str, &str)] = &[("\u{1F37A}", "\u{2713}"), ("\u{1F37B}", "\u{2713}")];

fn word(homebrew: &str) -> Option<&'static str> {
    match homebrew {
        "brew" | "homebrew" => Some(brand::NAME),
        _ => WORDS.iter().find(|(h, _)| *h == homebrew).map(|(_, o)| *o),
    }
}

/// Swaps each word of a hyphenated name: `no-cleanup-tap` -> `no-cleanup-follow`.
fn swap_parts(name: &str) -> String {
    name.split('-').map(|p| word(p).unwrap_or(p)).collect::<Vec<_>>().join("-")
}

/// The full list.
pub fn entries() -> Vec<Entry> {
    let mut all = Vec::new();
    let entry = |h: String, o: String, kind| Entry { homebrew: h, ours: o, kind };
    for name in HOMEBREW_COMMANDS {
        all.push(entry(name.to_string(), swap_parts(name), Kind::Command));
    }
    for name in HOMEBREW_FLAGS {
        all.push(entry(format!("--{name}"), format!("--{}", swap_parts(name)), Kind::Flag));
    }
    for (h, o) in WORDS.iter().chain(OUTPUT_WORDS) {
        all.push(entry(h.to_string(), o.to_string(), Kind::Word));
    }
    for h in ["brew", "homebrew"] {
        all.push(entry(h.to_string(), brand::NAME.to_string(), Kind::Word));
    }
    all
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn typed_words_map_one_to_one() {
        // Two Homebrew commands or flags must never share a replacement,
        // otherwise translating back would be ambiguous.
        for kind in [Kind::Command, Kind::Flag] {
            let mut seen = HashMap::new();
            for x in entries().iter().filter(|x| x.kind == kind) {
                if let Some(prev) = seen.insert(x.ours.clone(), x.homebrew.clone()) {
                    panic!("{} is used for both {} and {}", x.ours, prev, x.homebrew);
                }
            }
        }
    }

    #[test]
    fn replacements_are_clean() {
        let bad = ["brew", "keg", "tap", "cask", "formul", "bottle", "pour", "cellar"];
        for x in entries() {
            for b in bad {
                assert!(!x.ours.contains(b), "{} still contains {}", x.ours, b);
            }
        }
    }

    #[test]
    fn builds_names_from_words() {
        assert_eq!(swap_parts("no-cleanup-tap"), "no-cleanup-follow");
        assert_eq!(swap_parts("poured-from-bottle"), "extracted-from-prebuilt");
        assert_eq!(swap_parts("bump-formula-pr"), "bump-package-pr");
        assert_eq!(swap_parts("homebrew-prefixes"), "tote-prefixes");
    }
}
