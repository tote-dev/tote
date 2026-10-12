#!/bin/sh
# Installs the latest tote release for this computer.
#
#   curl -fsSL https://raw.githubusercontent.com/tote-dev/tote/main/install.sh | sh
#
# Settings:
#   TOTE_BIN_DIR       where to install (default: ~/.local/bin)
#   TOTE_VERSION       a release tag such as v0.1.0 (default: the latest)
#   TOTE_RELEASE_URL   download from here instead of GitHub releases
set -eu

NAME=tote
REPO=tote-dev/tote

BIN_DIR="${TOTE_BIN_DIR:-$HOME/.local/bin}"
if [ -n "${TOTE_VERSION:-}" ]; then
    BASE="https://github.com/$REPO/releases/download/$TOTE_VERSION"
else
    BASE="https://github.com/$REPO/releases/latest/download"
fi
BASE="${TOTE_RELEASE_URL:-$BASE}"

fail() {
    echo "$NAME install: $*" >&2
    exit 1
}

case "$(uname -s)-$(uname -m)" in
    Darwin-arm64) target=aarch64-apple-darwin ;;
    Darwin-x86_64) target=x86_64-apple-darwin ;;
    Linux-x86_64) target=x86_64-unknown-linux-gnu ;;
    Linux-aarch64 | Linux-arm64) target=aarch64-unknown-linux-gnu ;;
    *) fail "no prebuilt version for $(uname -s) $(uname -m). Build from source with: cargo install --path ." ;;
esac

archive="$NAME-$target.tar.gz"
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

echo "Downloading $archive"
curl -fsSL "$BASE/$archive" -o "$tmp/$archive" || fail "could not download $BASE/$archive"
curl -fsSL "$BASE/$archive.sha256" -o "$tmp/$archive.sha256" || fail "could not download the checksum"

expected="$(cut -d ' ' -f 1 < "$tmp/$archive.sha256")"
if command -v shasum > /dev/null 2>&1; then
    actual="$(shasum -a 256 "$tmp/$archive" | cut -d ' ' -f 1)"
else
    actual="$(sha256sum "$tmp/$archive" | cut -d ' ' -f 1)"
fi
[ "$expected" = "$actual" ] || fail "checksum mismatch, the download may be damaged"

tar -xzf "$tmp/$archive" -C "$tmp"
mkdir -p "$BIN_DIR"
install -m 755 "$tmp/$NAME" "$BIN_DIR/$NAME"
echo "Installed $NAME to $BIN_DIR/$NAME"

case ":$PATH:" in
    *":$BIN_DIR:"*) ;;
    *)
        echo
        echo "$BIN_DIR is not on your PATH. Add it with:"
        echo "  echo 'export PATH=\"$BIN_DIR:\$PATH\"' >> ~/.zshrc"
        echo "then open a new terminal."
        ;;
esac
