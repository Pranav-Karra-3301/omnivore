import { Download, Terminal, Package, Dock, CheckCircle, AlertCircle } from 'lucide-react'
import CodeBlock from '@/components/CodeBlock'

export default function InstallationPage() {
  return (
    <div className="prose prose-lg max-w-none">
      <h1 className="text-4xl font-bold text-gray-900 mb-6">Installation Guide</h1>

      <p className="text-xl text-gray-600 mb-8">
        Install Omnivore on macOS, Linux, or in Docker. Use Homebrew if you want the simplest setup; use Docker for containerized runs; or build from source for maximum control.
      </p>

      <div className="not-prose grid md:grid-cols-3 gap-6 my-12">
        <div className="bg-gradient-to-br from-blue-50 to-blue-100 rounded-xl p-6 border border-blue-200">
          <div className="w-12 h-12 bg-blue-500 rounded-lg flex items-center justify-center mb-4">
            <Package className="w-6 h-6 text-white" />
          </div>
          <h3 className="text-lg font-semibold text-gray-900 mb-2">Homebrew</h3>
          <p className="text-gray-600 text-sm">Easiest method for macOS and Linux users</p>
        </div>
        <div className="bg-gradient-to-br from-green-50 to-green-100 rounded-xl p-6 border border-green-200">
          <div className="w-12 h-12 bg-green-500 rounded-lg flex items-center justify-center mb-4">
            <Dock className="w-6 h-6 text-white" />
          </div>
          <h3 className="text-lg font-semibold text-gray-900 mb-2">Docker</h3>
          <p className="text-gray-600 text-sm">Containerized deployment for any platform</p>
        </div>
        <div className="bg-gradient-to-br from-purple-50 to-purple-100 rounded-xl p-6 border border-purple-200">
          <div className="w-12 h-12 bg-purple-500 rounded-lg flex items-center justify-center mb-4">
            <Terminal className="w-6 h-6 text-white" />
          </div>
          <h3 className="text-lg font-semibold text-gray-900 mb-2">From Source</h3>
          <p className="text-gray-600 text-sm">Build from source for maximum customization</p>
        </div>
      </div>

      <h2>Method 1: Homebrew (Recommended)</h2>
      <div className="not-prose bg-green-50 border border-green-200 rounded-lg p-4 mb-6">
        <div className="flex items-center">
          <CheckCircle className="w-5 h-5 text-green-600 mr-2" />
          <span className="text-green-800 font-medium">Recommended for most users</span>
        </div>
      </div>

      <h3>Install from Official Tap</h3>
      <CodeBlock language="bash">{`# Add the Omnivore tap
brew tap Pranav-Karra-3301/omnivore

# Install Omnivore
brew install omnivore`}</CodeBlock>

      <h3>Install from Local Formula</h3>
      <CodeBlock language="bash">{`# Clone the repository
git clone https://github.com/Pranav-Karra-3301/omnivore.git
cd omnivore

# Install from local formula
brew install --build-from-source ./Formula/omnivore.rb`}</CodeBlock>

      <h3>Verify</h3>
      <CodeBlock language="bash">{`# Check version
omnivore --version

# Help
omnivore --help`}</CodeBlock>

      <h2>Method 2: Docker</h2>
      <p>Docker provides a consistent environment across all platforms and includes all dependencies.</p>

      <h3>Quick Start</h3>
      <CodeBlock language="bash">{`# Run Omnivore CLI
docker run --rm -it omnivore:latest omnivore --help

# Run with local storage
docker run --rm -v $(pwd)/data:/var/lib/omnivore omnivore:latest`}</CodeBlock>

      <h3>Docker Compose</h3>
      <CodeBlock language="bash">{`# Clone the repository
git clone https://github.com/Pranav-Karra-3301/omnivore.git
cd omnivore

# Start the stack
docker-compose up -d`}</CodeBlock>

      <h2>Method 3: From Source</h2>
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

      <h3>Install Rust</h3>
      <CodeBlock language="bash">{`# Install Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Verify installation
rustc --version
cargo --version`}</CodeBlock>

      <h3>System Dependencies</h3>
      <p><strong>macOS</strong></p>
      <CodeBlock language="bash">{`brew install openssl pkg-config`}</CodeBlock>
      <p><strong>Ubuntu/Debian</strong></p>
      <CodeBlock language="bash">{`sudo apt update
sudo apt install -y build-essential pkg-config libssl-dev git`}</CodeBlock>
      <p><strong>RHEL/CentOS/Fedora</strong></p>
      <CodeBlock language="bash">{`sudo dnf install -y gcc openssl-devel pkg-config git
# or on older systems:
# sudo yum install -y gcc openssl-devel pkgconfig git`}</CodeBlock>

      <h3>Build and Install</h3>
      <CodeBlock language="bash">{`# Clone the repository
git clone https://github.com/Pranav-Karra-3301/omnivore.git
cd omnivore

# Build in release mode
cargo build --release

# Install CLI
cargo install --path omnivore-cli --force

# Or use the Makefile
make install`}</CodeBlock>

      <h3>Shell Completions</h3>
      <CodeBlock language="bash">{`# Generate completions
make completions

# Add to your shell profile (Bash)
echo 'source /path/to/omnivore/completions/omnivore.bash' >> ~/.bashrc

# Add to your shell profile (Zsh)
echo 'source /path/to/omnivore/completions/_omnivore' >> ~/.zshrc`}</CodeBlock>

      <h2>Configuration</h2>
      <p>After installation, configure Omnivore:</p>
      <h3>Create Default Config</h3>
      <CodeBlock language="bash">{`mkdir -p ~/.config/omnivore
omnivore config generate > ~/.config/omnivore/crawler.toml`}</CodeBlock>

      <h3>Example</h3>
      <CodeBlock language="toml">{`[crawler]
max_workers = 10
max_depth = 5
user_agent = "Omnivore/1.0"
respect_robots_txt = true

[crawler.politeness]
default_delay_ms = 100
max_requests_per_second = 10.0

[storage]
data_dir = "~/.local/share/omnivore"
cache_size_mb = 512`}</CodeBlock>

      <h2>Updating</h2>
      <h3>Homebrew</h3>
      <CodeBlock language="bash">{`brew update && brew upgrade omnivore`}</CodeBlock>
      <h3>Docker</h3>
      <CodeBlock language="bash">{`docker pull omnivore:latest
# Or rebuild
docker build -t omnivore:latest .`}</CodeBlock>
      <h3>From Source</h3>
      <CodeBlock language="bash">{`git pull origin main
cargo install --path omnivore-cli --force`}</CodeBlock>

      <h2>Uninstallation</h2>
      <h3>Homebrew</h3>
      <CodeBlock language="bash">{`brew services stop omnivore
brew uninstall omnivore
brew untap Pranav-Karra-3301/omnivore`}</CodeBlock>
      <h3>Cargo</h3>
      <CodeBlock language="bash">{`cargo uninstall omnivore
rm -rf ~/.local/share/omnivore
rm -rf ~/.config/omnivore`}</CodeBlock>
      <h3>Docker</h3>
      <CodeBlock language="bash">{`docker-compose down
docker rmi omnivore:latest
docker volume prune`}</CodeBlock>

      <h2>Troubleshooting</h2>
      <p><strong>Rust not found</strong></p>
      <CodeBlock language="bash">{`source ~/.cargo/env`}</CodeBlock>
      <p><strong>OpenSSL errors on macOS</strong></p>
      <CodeBlock language="bash">{`export OPENSSL_DIR=$(brew --prefix openssl)
export PKG_CONFIG_PATH="$OPENSSL_DIR/lib/pkgconfig"`}</CodeBlock>
      <p><strong>Permission denied</strong></p>
      <CodeBlock language="bash">{`echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc`}</CodeBlock>
    </div>
  )
}
