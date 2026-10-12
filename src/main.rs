mod args;
mod brand;
mod filter;
mod runner;
mod vocab;

use std::ffi::OsString;
use std::process::ExitCode;

fn main() -> ExitCode {
    let typed: Vec<OsString> = std::env::args_os().skip(1).collect();

    let Some(homebrew) = runner::find_homebrew() else {
        eprintln!(
            "{}: Homebrew was not found. Install it first, or point {}_BACKEND at its `brew` command.",
            brand::NAME,
            brand::ENV_PREFIX
        );
        return ExitCode::from(127);
    };

    if typed.len() == 1 && matches!(typed[0].to_str(), Some("--version" | "-v")) {
        println!("{} {}", brand::NAME, env!("CARGO_PKG_VERSION"));
    }

    let translated = args::Translator::new().translate(&typed);
    let code = runner::run(&homebrew, &translated);
    ExitCode::from(code.clamp(0, 255) as u8)
}
