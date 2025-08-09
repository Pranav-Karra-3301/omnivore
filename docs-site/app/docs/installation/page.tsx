import { 
  Download, 
  Terminal,
  Package,
  Docker,
  CheckCircle,
  AlertCircle,
  ExternalLink
} from 'lucide-react'

export default function InstallationPage() {
  return (
    <div className="prose prose-lg max-w-none">
      <h1 className="text-4xl font-bold text-gray-900 mb-6">
        Installation Guide
      </h1>
      
      <p className="text-xl text-gray-600 mb-8">
        Get Omnivore installed on your system with multiple installation methods. 
        Choose the one that works best for your environment.
      </p>

      {/* Installation Methods Grid */}
      <div className="not-prose grid md:grid-cols-3 gap-6 my-12">
        <div className="bg-gradient-to-br from-blue-50 to-blue-100 rounded-xl p-6 border border-blue-200">
          <div className="w-12 h-12 bg-blue-500 rounded-lg flex items-center justify-center mb-4">
            <Package className="w-6 h-6 text-white" />
          </div>
          <h3 className="text-lg font-semibold text-gray-900 mb-2">Homebrew</h3>
          <p className="text-gray-600 text-sm">
            Easiest method for macOS and Linux users
          </p>
        </div>

        <div className="bg-gradient-to-br from-green-50 to-green-100 rounded-xl p-6 border border-green-200">
          <div className="w-12 h-12 bg-green-500 rounded-lg flex items-center justify-center mb-4">
            <Docker className="w-6 h-6 text-white" />
          </div>
          <h3 className="text-lg font-semibold text-gray-900 mb-2">Docker</h3>
          <p className="text-gray-600 text-sm">
            Containerized deployment for any platform
          </p>
        </div>

        <div className="bg-gradient-to-br from-purple-50 to-purple-100 rounded-xl p-6 border border-purple-200">
          <div className="w-12 h-12 bg-purple-500 rounded-lg flex items-center justify-center mb-4">
            <Terminal className="w-6 h-6 text-white" />
          </div>
          <h3 className="text-lg font-semibold text-gray-900 mb-2">From Source</h3>
          <p className="text-gray-600 text-sm">
            Build from source for maximum customization
          </p>
        </div>
      </div>

      ## Method 1: Homebrew (Recommended)

      <div className="not-prose bg-green-50 border border-green-200 rounded-lg p-4 mb-6">
        <div className="flex items-center">
          <CheckCircle className="w-5 h-5 text-green-600 mr-2" />
          <span className="text-green-800 font-medium">Recommended for most users</span>
        </div>
      </div>

      Homebrew is the easiest way to install and manage Omnivore on macOS and Linux systems.

      ### Prerequisites
      - macOS 10.14+ or Linux with glibc 2.17+
      - [Homebrew](https://brew.sh/) installed

      ### Install from Official Tap

      ```bash
      # Add the Omnivore tap
      brew tap Pranav-Karra-3301/omnivore

      # Install Omnivore
      brew install omnivore
      ```

      ### Install from Formula File

      If you have the source code:

      ```bash
      # Clone the repository
      git clone https://github.com/Pranav-Karra-3301/omnivore.git
      cd omnivore

      # Install from local formula
      brew install --build-from-source ./Formula/omnivore.rb
      ```

      ### Verify Installation

      ```bash
      # Check version
      omnivore --version

      # Test basic functionality
      omnivore --help
      ```

      ### Service Management

      Omnivore includes a built-in API server that can be managed as a service:

      ```bash
      # Start the API server
      brew services start omnivore

      # Stop the API server
      brew services stop omnivore

      # Check service status
      brew services list | grep omnivore
      ```

      The API server will be available at `http://localhost:3000`.

      ## Method 2: Docker

      Docker provides a consistent environment across all platforms and includes all dependencies.

      ### Prerequisites
      - [Docker](https://www.docker.com/get-started) 20.10+
      - [Docker Compose](https://docs.docker.com/compose/) 2.0+ (optional)

      ### Quick Start with Docker

      ```bash
      # Run Omnivore CLI
      docker run --rm -it omnivore:latest omnivore --help

      # Run API server
      docker run --rm -p 3000:3000 omnivore:latest

      # Mount local directory for data persistence
      docker run --rm -v $(pwd)/data:/var/lib/omnivore -p 3000:3000 omnivore:latest
      ```

      ### Docker Compose (Recommended)

      For a full development stack with database and monitoring:

      ```bash
      # Clone the repository
      git clone https://github.com/Pranav-Karra-3301/omnivore.git
      cd omnivore

      # Start the full stack
      docker-compose up -d

      # Start with monitoring (Prometheus + Grafana)
      docker-compose --profile monitoring up -d

      # Start with browser automation
      docker-compose --profile browser up -d
      ```

      Services included:
      - **Omnivore API**: `http://localhost:3000`
      - **PostgreSQL**: `localhost:5432`
      - **Redis**: `localhost:6379`
      - **Prometheus** (optional): `http://localhost:9090`
      - **Grafana** (optional): `http://localhost:3001`

      ### Build from Source (Docker)

      ```bash
      # Clone and build
      git clone https://github.com/Pranav-Karra-3301/omnivore.git
      cd omnivore

      # Build Docker image
      docker build -t omnivore:local .

      # Run your build
      docker run --rm -p 3000:3000 omnivore:local
      ```

      ## Method 3: From Source

      Building from source gives you the latest features and allows customization.

      ### Prerequisites

      <div className="not-prose bg-blue-50 border border-blue-200 rounded-lg p-4 mb-6">
        <div className="flex items-start">
          <AlertCircle className="w-5 h-5 text-blue-600 mr-2 mt-0.5" />
          <div>
            <p className="text-blue-800 font-medium">System Requirements</p>
            <ul className="text-blue-700 text-sm mt-2 space-y-1">
              <li>• Rust 1.80+ with Cargo</li>
              <li>• Git for cloning the repository</li>
              <li>• OpenSSL development headers</li>
              <li>• pkg-config (Linux)</li>
            </ul>
          </div>
        </div>
      </div>

      #### Install Rust

      If you don't have Rust installed:

      ```bash
      # Install Rust via rustup
      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
      source ~/.cargo/env

      # Verify installation
      rustc --version
      cargo --version
      ```

      #### System Dependencies

      **macOS:**
      ```bash
      # Install OpenSSL (if needed)
      brew install openssl pkg-config
      ```

      **Ubuntu/Debian:**
      ```bash
      # Install required packages
      sudo apt update
      sudo apt install -y build-essential pkg-config libssl-dev git
      ```

      **RHEL/CentOS/Fedora:**
      ```bash
      # Install required packages
      sudo dnf install -y gcc openssl-devel pkg-config git
      # or on older systems:
      # sudo yum install -y gcc openssl-devel pkgconfig git
      ```

      ### Build and Install

      ```bash
      # Clone the repository
      git clone https://github.com/Pranav-Karra-3301/omnivore.git
      cd omnivore

      # Build in release mode
      cargo build --release

      # Install binaries
      cargo install --path omnivore-cli --force
      cargo install --path omnivore-api --force

      # Or use the Makefile
      make install
      ```

      ### Generate Shell Completions

      ```bash
      # Generate completions
      make completions

      # Add to your shell profile
      echo 'source /path/to/omnivore/completions/omnivore.bash' >> ~/.bashrc  # Bash
      echo 'source /path/to/omnivore/completions/_omnivore' >> ~/.zshrc       # Zsh
      ```

      ### Verify Installation

      ```bash
      # Check version
      omnivore --version

      # Check API server
      omnivore-api --help

      # Run tests
      cargo test --all
      ```

      ## Configuration

      After installation, you can configure Omnivore:

      ### Default Configuration

      Omnivore looks for configuration in these locations:
      1. `~/.config/omnivore/crawler.toml` (user config)
      2. `/etc/omnivore/crawler.toml` (system config)  
      3. `./crawler.toml` (local config)

      ### Create Default Config

      ```bash
      # Create config directory
      mkdir -p ~/.config/omnivore

      # Generate default config
      omnivore config generate > ~/.config/omnivore/crawler.toml
      ```

      Example configuration:

      ```toml
      [crawler]
      max_workers = 10
      max_depth = 5
      user_agent = "Omnivore/1.0"
      respect_robots_txt = true

      [crawler.politeness]
      default_delay_ms = 100
      max_requests_per_second = 10.0

      [storage]
      data_dir = "~/.local/share/omnivore"
      cache_size_mb = 512

      [api]
      host = "127.0.0.1"
      port = 3000
      ```

      ## Updating

      ### Homebrew

      ```bash
      # Update Homebrew and upgrade Omnivore
      brew update && brew upgrade omnivore
      ```

      ### Docker

      ```bash
      # Pull latest image
      docker pull omnivore:latest

      # Or rebuild from source
      docker build -t omnivore:latest .
      ```

      ### From Source

      ```bash
      # Pull latest changes
      git pull origin main

      # Rebuild and install
      cargo install --path omnivore-cli --force
      cargo install --path omnivore-api --force
      ```

      ## Uninstallation

      ### Homebrew

      ```bash
      # Stop services
      brew services stop omnivore

      # Uninstall
      brew uninstall omnivore

      # Remove tap (optional)
      brew untap Pranav-Karra-3301/omnivore
      ```

      ### Cargo (From Source)

      ```bash
      # Uninstall binaries
      cargo uninstall omnivore
      cargo uninstall omnivore-api

      # Remove data (optional)
      rm -rf ~/.local/share/omnivore
      rm -rf ~/.config/omnivore
      ```

      ### Docker

      ```bash
      # Stop containers
      docker-compose down

      # Remove images
      docker rmi omnivore:latest

      # Remove volumes (optional - this deletes data!)
      docker volume prune
      ```

      ## Troubleshooting

      ### Common Issues

      **Rust not found:**
      ```bash
      # Make sure Rust is in your PATH
      source ~/.cargo/env
      ```

      **OpenSSL errors on macOS:**
      ```bash
      # Set environment variables
      export OPENSSL_DIR=$(brew --prefix openssl)
      export PKG_CONFIG_PATH="$OPENSSL_DIR/lib/pkgconfig"
      ```

      **Permission denied:**
      ```bash
      # On Linux, you may need to add cargo bin to PATH
      echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
      source ~/.bashrc
      ```

      ### Getting Help

      If you encounter issues:

      1. Check the [Troubleshooting Guide](/guides/troubleshooting)
      2. Search [existing issues](https://github.com/Pranav-Karra-3301/omnivore/issues)
      3. Create a [new issue](https://github.com/Pranav-Karra-3301/omnivore/issues/new) with:
         - Your operating system and version
         - Installation method used
         - Complete error messages
         - Output of `omnivore --version`

      ## Next Steps

      Now that Omnivore is installed:

      1. **[Quick Start Guide](/docs/quickstart)** - Start crawling immediately
      2. **[Configuration](/docs/configuration)** - Customize for your needs  
      3. **[CLI Reference](/docs/cli)** - Learn all available commands
      4. **[Examples](/examples)** - See real-world usage patterns

      <div className="not-prose bg-gray-50 rounded-lg p-6 mt-8">
        <h3 className="text-lg font-semibold text-gray-900 mb-4">✨ Pro Tip</h3>
        <p className="text-gray-700">
          Start with Homebrew for the easiest installation experience. You can always switch to 
          Docker or source builds later for more advanced use cases.
        </p>
      </div>
    </div>
  )
}