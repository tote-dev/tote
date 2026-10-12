class Tote < Formula
  desc "Drop-in front end for Homebrew with plain words instead of brewing terms"
  homepage "https://github.com/tote-dev/tote"
  version "0.1.0"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/tote-dev/tote/releases/download/v0.1.0/tote-aarch64-apple-darwin.tar.gz"
      sha256 "7f3ef1f947e5b8200c7346fabb183c1af974d35ced00d9d7f5857ed62343a9d0"
    end
    on_intel do
      url "https://github.com/tote-dev/tote/releases/download/v0.1.0/tote-x86_64-apple-darwin.tar.gz"
      sha256 "36ea868ddb13dd54672d8acc5b6ff99b426805c7ffe9e673670573175141352c"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/tote-dev/tote/releases/download/v0.1.0/tote-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "1495222a585e7767e350f31bb587be94ccd48a2ddcce9f1d3bbf5708d19c7c5d"
    end
    on_intel do
      url "https://github.com/tote-dev/tote/releases/download/v0.1.0/tote-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "7f15b8da83f22f58b3de0a7b88546a38c0b2c1eb0b7449f253b9b92baafe363f"
    end
  end

  def install
    bin.install "tote"
  end

  test do
    assert_match "tote #{version}", shell_output("#{bin}/tote --version")
  end
end
