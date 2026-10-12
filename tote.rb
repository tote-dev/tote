class Tote < Formula
  desc "Drop-in front end for Homebrew with plain words instead of brewing terms"
  homepage "https://github.com/tote-dev/tote"
  version "0.1.0"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/tote-dev/tote/releases/download/v0.1.0/tote-aarch64-apple-darwin.tar.gz"
      sha256 "011c78c5894dc00be7f913480f6c829467e295f5f3bf30ba9630d4710acccfc1"
    end
    on_intel do
      url "https://github.com/tote-dev/tote/releases/download/v0.1.0/tote-x86_64-apple-darwin.tar.gz"
      sha256 "2101a91b784597764ebccb3f7dd7278731d357214bb0bcdb72f9537b9ce9261c"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/tote-dev/tote/releases/download/v0.1.0/tote-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "c678be2515cbfacf42af7cae5cd0c22f256467070d226cc46208b9891b744475"
    end
    on_intel do
      url "https://github.com/tote-dev/tote/releases/download/v0.1.0/tote-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "b7d1810fd4a9a4e8896b9b73b86e651201b29cfa89c6091ca593418d9e3e2a15"
    end
  end

  def install
    bin.install "tote"
  end

  test do
    assert_match "tote #{version}", shell_output("#{bin}/tote --version")
  end
end
