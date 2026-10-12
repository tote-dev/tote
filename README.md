# tote

**Homebrew, without the beer.**

Homebrew is how most of us install software on a Mac. It also greets you with a 🍺 after every install, asks you to "pour bottles" into your "Cellar", and calls its sources "taps".

That's charming if you drink. It's less charming if you don't: because you're in recovery, because of your faith, because of your health, or just because. And you see it every time you install something.

tote is Homebrew with plain words. Same packages, same commands. Just no bar talk.

```
$ brew install wget
==> Pouring wget--1.24.5.bottle.tar.gz
🍺  /opt/homebrew/Cellar/wget/1.24.5: 92 files, 4.5MB

$ tote install wget
==> Extracting wget--1.24.5.bottle.tar.gz
✓  /opt/homebrew/Cellar/wget/1.24.5: 92 files, 4.5MB
```

## Install

You need [Homebrew](https://brew.sh). Then:

```
brew tap tote-dev/tote https://github.com/tote-dev/tote
brew trust tote-dev/tote
brew install tote
```

That's the last time you'll need to type `brew`. From now on, `tote upgrade` keeps tote up to date along with everything else.

Prefer not to go through Homebrew? This one-liner installs tote into `~/.local/bin`:

```
curl -fsSL https://raw.githubusercontent.com/tote-dev/tote/main/install.sh | sh
```

## Use it

Anything you'd do with `brew`, do with `tote`:

```
tote install wget
tote upgrade
tote search ffmpeg
tote list
```

When a guide online uses a brewing word, swap in the plain one:

| Homebrew says | tote says |
|---|---|
| `brew install --cask zoom` | `tote install --app zoom` |
| `brew tap user/repo` | `tote follow user/repo` |
| brew, Homebrew | tote, Tote |
| formula | package |
| cask | app |
| bottle | prebuilt |
| keg | folder |
| Cellar | Store |
| Caskroom | Approom |
| Brewfile | Bundlefile |
| pour, pouring | extract, extracting |
| brewing, brewed | building, built |
| 🍺, 🍻 | ✓ |

Forgot the new word? The old one still works. `tote tap user/repo` does the same thing as `tote follow user/repo`.

The full list, with every plural and flag, is in [VOCABULARY.md](VOCABULARY.md).

## Questions

**Is this a fork of Homebrew?**
No. tote runs your real Homebrew underneath and changes only the words you see and type. Homebrew updates keep working, and you can still use `brew` whenever you like.

**Will it break my setup?**
No. Your packages and folders stay exactly where Homebrew puts them, and package names, repo names, URLs and script output are never touched.

**Isn't a shell alias enough?**
An alias renames `brew`. It doesn't change "Pouring bottle", "keg-only", the 🍺, or the "run `brew tap`" hints in error messages. tote translates all of it, both ways.

**I want Homebrew's exact output for a moment.**
Run any command with `TOTE_RAW=1` in front of it.

## More

- [How it works](HOW_IT_WORKS.md): settings, building from source, what is and isn't translated, and how to contribute.
- [VOCABULARY.md](VOCABULARY.md): every word, and why it was chosen.

Ideas for better words are welcome. Open an issue.

## License

MIT. See [LICENSE](LICENSE). tote is not affiliated with or endorsed by the Homebrew project.
