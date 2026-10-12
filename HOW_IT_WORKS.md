# How tote works

tote is a small Rust program that sits in front of Homebrew. When you run a `tote` command, it:

1. Translates the words you typed back into Homebrew's (`follow` becomes `tap`, `--app` becomes `--cask`, and so on).
2. Runs your real `brew` with them.
3. Translates Homebrew's output, including error messages and the hints it prints, into tote's words as it streams past.

Nothing in Homebrew is modified, so Homebrew updates keep working.

```
brew install wget                 ->  tote install wget
brew install --cask zoom          ->  tote install --app zoom
brew tap user/repo                ->  tote follow user/repo
brew untap user/repo              ->  tote unfollow user/repo
brew list --formula               ->  tote list --package
brew --cellar                     ->  tote --store
HOMEBREW_NO_AUTO_UPDATE=1 brew …  ->  TOTE_NO_AUTO_UPDATE=1 tote …
```

```
==> Pouring wget--1.24.5.bottle.tar.gz   ->  ==> Extracting wget--1.24.5.bottle.tar.gz
🍺  /opt/homebrew/Cellar/wget/1.24.5      ->  ✓  /opt/homebrew/Cellar/wget/1.24.5
No available formula ... `brew tap` it?   ->  No available package ... `tote follow` it?
```

## The words

| Homebrew | tote |
|---|---|
| brew / Homebrew | tote / Tote |
| tap, untap | follow, unfollow |
| formula | package |
| cask | app |
| bottle | prebuilt |
| keg, keg-only | folder, folder-only |
| Cellar | Store |
| Caskroom | Approom |
| Brewfile | Bundlefile |
| pour | extract |
| brewing, brewed | building, built |
| 🍺 | ✓ |
| `HOMEBREW_*` settings | `TOTE_*` |

[VOCABULARY.md](VOCABULARY.md) lists every form (plurals, flags, compound commands) and why each word was chosen. The words you type still work in Homebrew's spelling too, so `tote tap user/repo` works.

## What stays the same

- **Folders on disk.** Homebrew's prebuilt packages depend on paths like `/opt/homebrew/Cellar`, so those paths are printed as they are, and you can copy them and `cd` there. Set `TOTE_HIDE_PATHS=1` to swap the words inside printed paths too (the paths shown then don't exist on disk).
- **Names.** Package names, repo names like `homebrew/core`, and URLs are never changed.
- **Scripts.** JSON output (`--json`) and commands meant for shells or scripts (`shellenv`, `--prefix`, `--env`, `sh`, `edit` and similar) run untouched.
- **Exit codes**, prompts, colours and Ctrl-C behave as they do in Homebrew.

## Settings

| Setting | Effect |
|---|---|
| `TOTE_RAW=1` | Show Homebrew's output exactly as it is. |
| `TOTE_HIDE_PATHS=1` | Also swap words inside printed paths. |
| `TOTE_BACKEND=/path/to/brew` | Use a specific Homebrew. |
| `TOTE_<anything else>` | Passed to Homebrew as `HOMEBREW_<anything else>`, unless that is already set. |

tote finds Homebrew on your `PATH`, or in its standard locations. To use a specific one, set `TOTE_BACKEND=/path/to/brew`.

## Other ways to install

The one-line installer downloads the prebuilt tote for your computer (Mac with Apple Silicon or Intel, or Linux):

```
curl -fsSL https://raw.githubusercontent.com/tote-dev/tote/main/install.sh | sh
```

It puts `tote` in `~/.local/bin` (set `TOTE_BIN_DIR` to choose another folder) and checks the download against its published checksum.

To build from source instead, install [Rust](https://rustup.rs) and run:

```
git clone https://github.com/tote-dev/tote
cd tote
cargo install --path .
```

## Renaming

The tool's name lives in one place. To rename it:

1. Edit `src/brand.rs` (the command name and settings prefix).
2. Change `name` in `Cargo.toml`, `BIN` in `.github/workflows/release.yml`, `NAME` in `install.sh`, and `NAME` and `CLASS` in `scripts/write-formula.sh`.
3. Update the name in README.md, this file and VOCABULARY.md.

The word list is in `src/vocab.rs`. Command and flag names are built from it: every Homebrew command or flag containing one of the words is translated word by word, so `--no-cleanup-tap` becomes `--no-cleanup-follow`. A test checks that no two commands or flags share a replacement, and that no replacement contains a brewing word.

## Development

```
cargo test
```

The tests run the real binary against a stand-in Homebrew (`tests/fixtures/brew`), so they don't need Homebrew installed.

To publish a release, bump `version` in `Cargo.toml`, then tag and push:

```
git tag v0.1.0
git push origin v0.1.0
```

GitHub Actions builds tote for each platform, attaches the downloads to a release, and updates the brew recipe (`tote.rb`) to point at them. `install.sh` and `brew install` then pick up the new version.
