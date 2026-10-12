//! Rewrites Homebrew's output, swapping each Homebrew word for ours.
//!
//! Works on raw bytes so non-UTF-8 output passes through untouched. A word is
//! only swapped when it stands on its own:
//! - colour codes are skipped, so they never glue onto a word;
//! - tokens containing `/` (paths, repo names, URLs) are kept as-is, so they can
//!   still be copied and used (unless path hiding is on);
//! - a word that is part of a longer name (`brew-gem`, `x.bottle.tar.gz`,
//!   `python@3.12`) is left alone.

use crate::brand;
use crate::vocab;
use std::collections::HashMap;

pub struct Filter {
    words: HashMap<String, String>,
    hide_paths: bool,
}

const SETTINGS_PREFIX: &[u8] = b"HOMEBREW_";

impl Filter {
    pub fn new(hide_paths: bool) -> Self {
        let words = vocab::entries()
            .into_iter()
            .map(|x| (x.homebrew.to_string(), x.ours))
            .collect();
        Filter { words, hide_paths }
    }

    /// Rewrites one chunk of output. Chunks should end at a line break
    /// (or at least not in the middle of a word).
    pub fn rewrite(&self, input: &[u8]) -> Vec<u8> {
        let mut out = Vec::with_capacity(input.len() + 16);
        let mut i = 0;
        let mut text_start = 0;
        while i < input.len() {
            if input[i] == 0x1b
                && let Some(end) = ansi_end(input, i) {
                    self.rewrite_text(&input[text_start..i], &mut out);
                    out.extend_from_slice(&input[i..end]);
                    i = end;
                    text_start = end;
                    continue;
                }
            i += 1;
        }
        self.rewrite_text(&input[text_start..], &mut out);
        replace_emoji(out)
    }

    fn rewrite_text(&self, text: &[u8], out: &mut Vec<u8>) {
        let mut i = 0;
        while i < text.len() {
            if text[i].is_ascii_whitespace() {
                out.push(text[i]);
                i += 1;
                continue;
            }
            let start = i;
            while i < text.len() && !text[i].is_ascii_whitespace() {
                i += 1;
            }
            let token = &text[start..i];
            if contains(token, b"://") {
                out.extend_from_slice(token);
            } else if token.contains(&b'/') {
                if self.hide_paths {
                    self.rewrite_path(token, out);
                } else {
                    out.extend_from_slice(token);
                }
            } else {
                self.rewrite_token(token, out);
            }
        }
    }

    fn rewrite_token(&self, token: &[u8], out: &mut Vec<u8>) {
        let mut i = 0;
        while i < token.len() {
            if !is_word_byte(token[i]) {
                out.push(token[i]);
                i += 1;
                continue;
            }
            let start = i;
            while i < token.len()
                && (is_word_byte(token[i])
                    || (token[i] == b'.'
                        && i > start
                        && token.get(i + 1).is_some_and(|b| b.is_ascii_alphanumeric())))
            {
                i += 1;
            }
            self.rewrite_word(&token[start..i], out);
        }
    }

    fn rewrite_word(&self, word: &[u8], out: &mut Vec<u8>) {
        if word.starts_with(SETTINGS_PREFIX) && word.len() > SETTINGS_PREFIX.len() {
            out.extend_from_slice(brand::ENV_PREFIX.as_bytes());
            out.push(b'_');
            out.extend_from_slice(&word[SETTINGS_PREFIX.len()..]);
            return;
        }
        let lower = String::from_utf8_lossy(word).to_ascii_lowercase();
        match self.words.get(&lower) {
            Some(ours) => out.extend_from_slice(match_case(word, ours).as_bytes()),
            None => out.extend_from_slice(word),
        }
    }

    fn rewrite_path(&self, token: &[u8], out: &mut Vec<u8>) {
        for (n, part) in token.split(|b| *b == b'/').enumerate() {
            if n > 0 {
                out.push(b'/');
            }
            let lower = String::from_utf8_lossy(part).to_ascii_lowercase();
            let ours = match lower.as_str() {
                "homebrew" | "linuxbrew" => Some(brand::NAME),
                "cellar" | "caskroom" | "taps" => self.words.get(&lower).map(|s| s.as_str()),
                _ => None,
            };
            match ours {
                Some(o) => out.extend_from_slice(match_case(part, o).as_bytes()),
                None => self.rewrite_token(part, out),
            }
        }
    }
}

fn is_word_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b'-' || b == b'@' || b >= 0x80
}

fn contains(hay: &[u8], needle: &[u8]) -> bool {
    hay.windows(needle.len()).any(|w| w == needle)
}

/// Copies the case of the original word: lower, Capitalized or UPPER.
fn match_case(original: &[u8], ours: &str) -> String {
    let letters: Vec<u8> = original.iter().copied().filter(u8::is_ascii_alphabetic).collect();
    if letters.len() > 1 && letters.iter().all(u8::is_ascii_uppercase) {
        return ours.to_ascii_uppercase();
    }
    let lower = ours.to_ascii_lowercase();
    if letters.first().is_some_and(u8::is_ascii_uppercase) {
        // Capitalise the first letter, skipping leading dashes.
        let mut done = false;
        lower
            .chars()
            .map(|c| {
                if !done && c.is_ascii_alphabetic() {
                    done = true;
                    c.to_ascii_uppercase()
                } else {
                    c
                }
            })
            .collect()
    } else {
        lower
    }
}

/// Returns the end of the escape sequence starting at `i`, if it is one.
fn ansi_end(s: &[u8], i: usize) -> Option<usize> {
    match s.get(i + 1)? {
        b'[' => {
            let mut j = i + 2;
            while j < s.len() {
                if (0x40..=0x7e).contains(&s[j]) {
                    return Some(j + 1);
                }
                j += 1;
            }
            None
        }
        b']' => {
            let mut j = i + 2;
            while j < s.len() {
                if s[j] == 0x07 {
                    return Some(j + 1);
                }
                if s[j] == 0x1b && s.get(j + 1) == Some(&b'\\') {
                    return Some(j + 2);
                }
                j += 1;
            }
            None
        }
        _ => Some(i + 2),
    }
}

fn replace_emoji(mut out: Vec<u8>) -> Vec<u8> {
    for (from, to) in vocab::EMOJI {
        let (from, to) = (from.as_bytes(), to.as_bytes());
        if !contains(&out, from) {
            continue;
        }
        let mut next = Vec::with_capacity(out.len());
        let mut i = 0;
        while i < out.len() {
            if out[i..].starts_with(from) {
                next.extend_from_slice(to);
                i += from.len();
            } else {
                next.push(out[i]);
                i += 1;
            }
        }
        out = next;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: &str) -> String {
        String::from_utf8(Filter::new(false).rewrite(s.as_bytes())).unwrap()
    }

    #[test]
    fn swaps_words() {
        assert_eq!(
            run("==> Pouring wget--1.24.5.arm64_sonoma.bottle.tar.gz\n"),
            "==> Extracting wget--1.24.5.arm64_sonoma.bottle.tar.gz\n"
        );
        assert_eq!(run("==> Tapping homebrew/core\n"), "==> Following homebrew/core\n");
        assert_eq!(run("openssl@3 is keg-only, which means"), "openssl@3 is folder-only, which means");
        assert_eq!(run("No available formula with the name"), "No available package with the name");
        assert_eq!(run("Homebrew 4.4.0"), "Tote 4.4.0");
        assert_eq!(run("Brewfile"), "Bundlefile");
    }

    #[test]
    fn swaps_command_hints() {
        assert_eq!(run("Run `brew untap foo/bar` to remove."), "Run `tote unfollow foo/bar` to remove.");
        assert_eq!(run("Use `brew install --cask zoom`."), "Use `tote install --app zoom`.");
        assert_eq!(run("brew --cellar"), "tote --store");
    }

    #[test]
    fn swaps_setting_names() {
        assert_eq!(
            run("Hide these hints with HOMEBREW_NO_ENV_HINTS (see `man brew`)."),
            "Hide these hints with TOTE_NO_ENV_HINTS (see `man tote`)."
        );
    }

    #[test]
    fn keeps_paths_urls_and_names() {
        let line = "/opt/homebrew/Cellar/wget/1.24.5: 92 files https://github.com/Homebrew/brew\n";
        assert_eq!(run(line), line);
        assert_eq!(run("brew-gem 2.0"), "brew-gem 2.0");
        assert_eq!(run("homebrew/cask"), "homebrew/cask");
    }

    #[test]
    fn keeps_colour_codes() {
        assert_eq!(
            run("\x1b[34m==>\x1b[0m \x1b[1mPouring wget\x1b[0m\n"),
            "\x1b[34m==>\x1b[0m \x1b[1mExtracting wget\x1b[0m\n"
        );
    }

    #[test]
    fn matches_case() {
        assert_eq!(run("CASKS"), "APPS");
        assert_eq!(run("Casks:"), "Apps:");
        assert_eq!(run("casks"), "apps");
    }

    #[test]
    fn swaps_emoji() {
        assert_eq!(run("\u{1F37A}  /opt/homebrew/Cellar/wget/1.24.5"), "\u{2713}  /opt/homebrew/Cellar/wget/1.24.5");
    }

    #[test]
    fn hides_paths_when_asked() {
        let f = Filter::new(true);
        let out = f.rewrite(b"/opt/homebrew/Cellar/wget/1.24.5 /opt/homebrew/Caskroom/zoom");
        assert_eq!(String::from_utf8(out).unwrap(), "/opt/tote/Store/wget/1.24.5 /opt/tote/Approom/zoom");
    }
}
