class Heft < Formula
  desc "hn: A minimal hackernews CLI"
  homepage "https://www.github.com/1ntEgr8/hn"
  url "https://github.com/1ntEgr8/hn/releases/download/v0.1.0/hn-mac.tar.gz"
  version "0.1.0"
  sha256 "b1976c93e61388ef8c2fbfe34cba6fb59a41ddd95a69a16dbc9c6988cf60fdb2"

  def install
    bin.install "hn"
  end
end
