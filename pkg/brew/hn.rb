class HnBin < Formula
    version '0.1.0'
    desc "A minimal HackerNews CLI"
    homepage "https://www.github.com/1ntEgr8/hn"

    if OS.mac?
        url "https://github.com/1ntEgr8/hn/releases/download/v0.1.0/hn-mac.tar.gz"
        sha256 "b1976c93e61388ef8c2fbfe34cba6fb59a41ddd95a69a16dbc9c6988cf60fdb2"
    end

    def install
        bin.install "hn"

    end
end

