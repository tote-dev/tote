//! Everything that names this tool lives here, so renaming it is a one-file
//! change (plus the `name` in Cargo.toml). See HOW_IT_WORKS.md "Renaming".

/// The command users type, and the word that replaces "brew" and "Homebrew"
/// in output (capitalised to match, so "Homebrew" becomes "Tote").
pub const NAME: &str = "tote";

/// Prefix of this tool's own settings, e.g. TOTE_RAW. Any other
/// `<ENV_PREFIX>_X` variable is passed to Homebrew as `HOMEBREW_X`.
pub const ENV_PREFIX: &str = "TOTE";
