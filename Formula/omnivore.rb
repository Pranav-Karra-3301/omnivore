class Omnivore < Formula
  desc "Universal Rust Web Crawler & Knowledge Graph Builder"
  homepage "https://github.com/yourusername/omnivore"
  version "0.1.0"
  license "MIT OR Apache-2.0"
  head "https://github.com/yourusername/omnivore.git", branch: "main"

  # Release tarball (update when you create releases)
  if OS.mac?
    if Hardware::CPU.arm?
      url "https://github.com/yourusername/omnivore/releases/download/v#{version}/omnivore-v#{version}-aarch64-apple-darwin.tar.gz"
      sha256 "REPLACE_WITH_ARM64_MAC_SHA256"
    else
      url "https://github.com/yourusername/omnivore/releases/download/v#{version}/omnivore-v#{version}-x86_64-apple-darwin.tar.gz"  
      sha256 "REPLACE_WITH_X86_64_MAC_SHA256"
    end
  elsif OS.linux?
    if Hardware::CPU.arm?
      url "https://github.com/yourusername/omnivore/releases/download/v#{version}/omnivore-v#{version}-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "REPLACE_WITH_ARM64_LINUX_SHA256"  
    else
      url "https://github.com/yourusername/omnivore/releases/download/v#{version}/omnivore-v#{version}-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "REPLACE_WITH_X86_64_LINUX_SHA256"
    end
  end

  # Build from source option
  option "with-source", "Build from source instead of using precompiled binaries"

  depends_on "rust" => :build if build.with?("source")

  def install
    if build.with?("source") || build.head?
      # Build from source
      system "cargo", "install", "--locked", "--root", prefix, "--path", "omnivore-cli"
      system "cargo", "install", "--locked", "--root", prefix, "--path", "omnivore-api"
      
      # Install configuration files
      etc.install "configs/crawler.toml" => "omnivore/crawler.toml"
      
      # Install documentation
      doc.install "README.md"
      doc.install Dir["docs/*"] if Dir.exist?("docs")
    else
      # Install precompiled binaries
      bin.install "omnivore"
      bin.install "omnivore-api"
      
      # Install configuration files
      etc.install "crawler.toml" => "omnivore/crawler.toml"
    end

    # Create data directory
    (var/"omnivore").mkpath
    
    # Install shell completions if available
    if (buildpath/"completions").exist?
      bash_completion.install "completions/omnivore.bash" => "omnivore"
      zsh_completion.install "completions/_omnivore"
      fish_completion.install "completions/omnivore.fish"
    end
  end

  service do
    run [opt_bin/"omnivore-api"]
    keep_alive true
    log_path var/"log/omnivore/api.log"
    error_log_path var/"log/omnivore/api.error.log"
    environment_variables({
      "OMNIVORE_CONFIG" => etc/"omnivore/crawler.toml",
      "OMNIVORE_DATA_DIR" => var/"omnivore"
    })
    working_dir var/"omnivore"
  end

  def post_install
    # Create log directory
    (var/"log/omnivore").mkpath
    
    # Set up default configuration if not exists
    config_file = etc/"omnivore/crawler.toml"
    unless config_file.exist?
      config_file.write default_config
    end
  end

  def default_config
    <<~EOS
      [crawler]
      max_workers = 10
      max_depth = 5
      user_agent = "Omnivore/#{version} (Homebrew)"
      respect_robots_txt = true

      [crawler.politeness]
      default_delay_ms = 100
      max_requests_per_second = 10.0

      [storage]
      data_dir = "#{var}/omnivore"
      cache_size_mb = 512

      [api]
      host = "127.0.0.1"
      port = 3000
    EOS
  end

  test do
    # Test CLI version
    assert_match version.to_s, shell_output("#{bin}/omnivore --version")
    
    # Test CLI help
    system bin/"omnivore", "--help"
    
    # Test API binary exists and shows help
    system bin/"omnivore-api", "--help"
    
    # Test basic functionality
    output = shell_output("#{bin}/omnivore stats 2>&1")
    assert_match(/No active sessions|statistics/i, output)
    
    # Test configuration file exists
    assert_predicate etc/"omnivore/crawler.toml", :exist?
  end

  def caveats
    <<~EOS
      Configuration file installed to:
        #{etc}/omnivore/crawler.toml

      Data directory:
        #{var}/omnivore

      To start the API server:
        brew services start omnivore

      Or run manually:
        omnivore-api

      For more information:
        omnivore --help
        omnivore-api --help
    EOS
  end
end