# tote vocabulary

**The rules:** words are chosen for how clearly they describe the job, not for a theme. Each offending word has exactly one replacement word, used everywhere: in commands, flags, settings and printed output. Word forms follow the base word (plural, -ing, -ed, `un-`, `-only`). Nothing else changes, so any brew command you find online becomes a tote command by swapping words, and tote swaps them back before calling Homebrew.

```
brew untap user/repo              ->  tote unfollow user/repo
brew install --cask zoom          ->  tote install --app zoom
brew install --force-bottle wget  ->  tote install --force-prebuilt wget
brew install wget                 ->  tote install wget      (nothing to swap)
```

## The word list

| # | Homebrew word | tote word | Other candidates |
|---|---|---|---|
| 1 | brew | **tote** | (the name, swappable later) |
| 2 | Homebrew | **Tote** | |
| 3 | tap | **follow** | mount, plug, hook |
| 4 | cask | **app** | gui, desktop |
| 5 | formula | **package** | pkg, tool, recipe |
| 6 | bottle | **prebuilt** | binary, pack |
| 7 | keg | **folder** | dir, slot |
| 8 | Cellar | **Store** | Depot, Shelf |
| 9 | Caskroom | **Approom** | Appdata, Appcache |
| 10 | Brewfile | **Bundlefile** | Totefile, Packagefile |
| 11 | pour | **extract** | unpack |
| 12 | brewing / brewed | **building / built** | making / made |
| 13 | 🍺 | **✓** | ✅, 📦, nothing |

`bundle` stays as it is, because it isn't a drinking word.

## Every form, derived from the list

| Homebrew | tote |
|---|---|
| `tap`, `untap`, `tap-info`, `tap-new` | `follow`, `unfollow`, `follow-info`, `follow-new` |
| taps, tapped, tapping, untapping | follows, followed, following, unfollowing |
| `--cask`, `--casks`, `casks`, `bump-cask-pr` | `--app`, `--apps`, `apps`, `bump-app-pr` |
| `--formula`, `--formulae`, `formula`, `formulae`, `bump-formula-pr` | `--package`, `--packages`, `package`, `packages`, `bump-package-pr` |
| `bottle`, bottles, bottled, `unbottled` | `prebuilt`, prebuilts, prebuilt, `unprebuilt` |
| `--force-bottle`, `--build-bottle`, `--bottle-tag` | `--force-prebuilt`, `--build-prebuilt`, `--prebuilt-tag` |
| keg, kegs, keg-only | folder, folders, folder-only |
| `--cellar`, Cellar | `--store`, Store |
| `--caskroom`, Caskroom | `--approom`, Approom |
| Brewfile | Bundlefile (an existing Brewfile is still read) |
| `HOMEBREW_NO_AUTO_UPDATE` and every `HOMEBREW_*` setting | `TOTE_NO_AUTO_UPDATE` and so on (the old names keep working) |
| Pouring, poured | Extracting, extracted |
| "Run `brew untap x`" (hints in output) | "Run `tote unfollow x`" |

Every other Homebrew command or flag containing one of these words is translated the same way, word by word: `--no-cleanup-tap` becomes `--no-cleanup-follow`, and `--poured-from-bottle` becomes `--extracted-from-prebuilt`. The list of names comes from Homebrew's own source.

Replacements must not clash with an existing Homebrew command or flag. For example, `--appdir` is already a Homebrew flag, so Caskroom couldn't use it. One real flag already uses one of our words: `bump-python-resources-pr --packages` is left as it is. `extract` is also the name of a Homebrew developer command; it never clashes, because "pour" only appears in output.

## Left as they are on purpose

| Item | Example | Why |
|---|---|---|
| Folders on disk | `/opt/homebrew/Cellar/wget/1.24.5` | Homebrew's prebuilt packages depend on these exact paths. Paths stay unchanged so you can still copy them and `cd` there. An opt-in setting hides the words in printed paths. |
| Repo names | `homebrew/core`, `user/homebrew-tools` | Real identifiers you type and share. tote accepts them as-is. |
| URLs | `https://github.com/Homebrew/brew` | They have to keep working. |
| Package names | `brew-gem`, `beer` | Real names, so changing them would break installs. |
