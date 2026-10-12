#!/bin/sh
# Writes the brew install recipe (tote.rb) for a release, using the
# checksums of its downloads. Run by the release workflow:
#   scripts/write-formula.sh v0.1.0 <folder with the .sha256 files> > tote.rb
set -eu

NAME=tote
CLASS=Tote
REPO=tote-dev/tote
DESC="Drop-in front end for Homebrew with plain words instead of brewing terms"

tag="$1"
dir="$2"
version="${tag#v}"
base="https://github.com/$REPO/releases/download/$tag"

sha() {
    cut -d ' ' -f 1 < "$dir/$NAME-$1.tar.gz.sha256"
}

cat <<RUBY
class $CLASS < Formula
  desc "$DESC"
  homepage "https://github.com/$REPO"
  version "$version"
  license "MIT"

  on_macos do
    on_arm do
      url "$base/$NAME-aarch64-apple-darwin.tar.gz"
      sha256 "$(sha aarch64-apple-darwin)"
    end
    on_intel do
      url "$base/$NAME-x86_64-apple-darwin.tar.gz"
      sha256 "$(sha x86_64-apple-darwin)"
    end
  end

  on_linux do
    on_arm do
      url "$base/$NAME-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "$(sha aarch64-unknown-linux-gnu)"
    end
    on_intel do
      url "$base/$NAME-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "$(sha x86_64-unknown-linux-gnu)"
    end
  end

  def install
    bin.install "$NAME"
  end

  test do
    assert_match "$NAME #{version}", shell_output("#{bin}/$NAME --version")
  end
end
RUBY
