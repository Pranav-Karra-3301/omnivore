class Omnivore < Formula
  desc "Universal Rust Web Crawler & Knowledge Graph Builder"
  homepage "https://github.com/Pranav-Karra-3301/omnivore"
  url "https://github.com/Pranav-Karra-3301/omnivore/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "REPLACE_WITH_ACTUAL_SHA256"
  license "MIT OR Apache-2.0"

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--locked", "--root", prefix, "--path", "omnivore-cli"
    system "cargo", "install", "--locked", "--root", prefix, "--path", "omnivore-api"
  end

  service do
    run [opt_bin/"omnivore-api"]
    keep_alive true
    log_path var/"log/omnivore-api.log"
    error_log_path var/"log/omnivore-api.error.log"
    environment_variables PATH: std_service_path_env
  end

  test do
    system bin/"omnivore", "--version"
    system bin/"omnivore", "--help"
    
    # Test basic crawler functionality
    output = shell_output("#{bin}/omnivore stats 2>&1", 0)
    assert_match "No active sessions found", output
  end
end