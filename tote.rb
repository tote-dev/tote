class Tote < Formula
  desc "Drop-in front end for Homebrew with plain words instead of brewing terms"
  homepage "https://github.com/tote-dev/tote"
  version "0.1.0"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/tote-dev/tote/releases/download/v0.1.0/tote-aarch64-apple-darwin.tar.gz"
      sha256 "7627b3a1af848b08afb102b2b9dccfd237f30e4815056f33afeae38ba5d5972d"
    end
    on_intel do
      url "https://github.com/tote-dev/tote/releases/download/v0.1.0/tote-x86_64-apple-darwin.tar.gz"
      sha256 "de27469444a6e0e170bf2af590e41ea1481588d81eaf20f43768772d94e18463"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/tote-dev/tote/releases/download/v0.1.0/tote-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "35764995122bbe6f854c6a58e8eb81c572045c2484edf50c2e48be347541a3fb"
    end
    on_intel do
      url "https://github.com/tote-dev/tote/releases/download/v0.1.0/tote-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "ecc4f778b6059412466aa9b04d01254ce1a5169622b87494c562edc26db2f504"
    end
  end

  def install
    bin.install "tote"
  end

  test do
    assert_match "tote #{version}", shell_output("#{bin}/tote --version")
  end
end
