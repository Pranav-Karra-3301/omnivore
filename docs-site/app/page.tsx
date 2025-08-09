import Link from 'next/link'
import { 
  Zap, 
  Globe, 
  Network, 
  Shield, 
  Cpu, 
  Database,
  ArrowRight,
  Github,
  Book,
  Download,
  Star,
  Users,
  Rocket
} from 'lucide-react'
import Image from 'next/image'

export default function HomePage() {
  return (
    <div className="min-h-screen">
      {/* Navigation */}
      <nav className="bg-white/80 backdrop-blur-sm border-b border-gray-200 sticky top-0 z-50">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between h-16">
            <div className="flex items-center space-x-8">
              <div className="flex items-center space-x-2">
                <div className="w-8 h-8 bg-gradient-to-br from-blue-500 to-purple-600 rounded-lg flex items-center justify-center">
                  <Globe className="w-5 h-5 text-white" />
                </div>
                <span className="text-xl font-bold text-gray-900">Omnivore</span>
              </div>
              <div className="hidden md:flex space-x-6">
                <Link href="/docs" className="text-gray-600 hover:text-gray-900 transition-colors">
                  Documentation
                </Link>
                <Link href="/guides" className="text-gray-600 hover:text-gray-900 transition-colors">
                  Guides
                </Link>
                <Link href="/api" className="text-gray-600 hover:text-gray-900 transition-colors">
                  API Reference
                </Link>
                <Link href="/examples" className="text-gray-600 hover:text-gray-900 transition-colors">
                  Examples
                </Link>
              </div>
            </div>
            <div className="flex items-center space-x-4">
              <a
                href="https://github.com/yourusername/omnivore"
                target="_blank"
                rel="noopener noreferrer"
                className="text-gray-600 hover:text-gray-900 transition-colors"
              >
                <Github className="w-5 h-5" />
              </a>
              <Link
                href="/docs/installation"
                className="bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-700 transition-colors font-medium"
              >
                Get Started
              </Link>
            </div>
          </div>
        </div>
      </nav>

      {/* Hero Section */}
      <section className="relative py-20 px-4 sm:px-6 lg:px-8">
        <div className="max-w-7xl mx-auto">
          <div className="text-center">
            <div className="inline-flex items-center space-x-2 bg-blue-100 text-blue-800 px-3 py-1 rounded-full text-sm font-medium mb-6">
              <Rocket className="w-4 h-4" />
              <span>v1.0.0 - Production Ready</span>
            </div>
            <h1 className="text-4xl md:text-6xl font-bold text-gray-900 mb-6">
              Universal Web Crawler &<br />
              <span className="bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
                Knowledge Graph
              </span>
            </h1>
            <p className="text-xl text-gray-600 mb-8 max-w-3xl mx-auto">
              High-performance, parallel web crawler and knowledge graph system built in Rust. 
              Extract, analyze, and graph data from the web at scale with intelligent processing.
            </p>
            <div className="flex flex-col sm:flex-row gap-4 justify-center">
              <Link
                href="/docs/installation"
                className="inline-flex items-center px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors font-medium"
              >
                <Download className="w-5 h-5 mr-2" />
                Get Started
              </Link>
              <Link
                href="/docs"
                className="inline-flex items-center px-6 py-3 border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-50 transition-colors font-medium"
              >
                <Book className="w-5 h-5 mr-2" />
                Read Docs
              </Link>
              <a
                href="https://github.com/yourusername/omnivore"
                target="_blank"
                rel="noopener noreferrer"
                className="inline-flex items-center px-6 py-3 border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-50 transition-colors font-medium"
              >
                <Github className="w-5 h-5 mr-2" />
                View Source
              </a>
            </div>
          </div>
          
          {/* Demo Video/Image Placeholder */}
          <div className="mt-16 max-w-4xl mx-auto">
            <div className="bg-gray-900 rounded-xl p-8 shadow-2xl">
              <div className="bg-gray-800 rounded-lg p-4 font-mono text-sm">
                <div className="flex items-center space-x-2 mb-4">
                  <div className="w-3 h-3 bg-red-500 rounded-full"></div>
                  <div className="w-3 h-3 bg-yellow-500 rounded-full"></div>
                  <div className="w-3 h-3 bg-green-500 rounded-full"></div>
                  <span className="text-gray-400 ml-4">Terminal</span>
                </div>
                <div className="text-green-400">
                  <p>$ omnivore crawl https://example.com --workers 10 --depth 5</p>
                  <p className="text-blue-400 mt-2">🕸️  Omnivore Web Crawler</p>
                  <p className="text-gray-300 mt-1">Starting crawl from: https://example.com</p>
                  <p className="text-gray-300">Configuration:</p>
                  <p className="text-gray-300">  Workers: 10</p>
                  <p className="text-gray-300">  Max depth: 5</p>
                  <p className="text-green-400 mt-2">✨ [00:01:23] Crawled: 1,247 | Success: 1,201 | Failed: 46</p>
                  <p className="text-green-400">📊 Final Statistics: 1,247 URLs crawled in 1m 23s</p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section className="py-20 bg-white">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-16">
            <h2 className="text-3xl md:text-4xl font-bold text-gray-900 mb-4">
              Built for Scale and Performance
            </h2>
            <p className="text-xl text-gray-600 max-w-2xl mx-auto">
              Omnivore combines cutting-edge Rust performance with intelligent crawling strategies
            </p>
          </div>
          
          <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
            <div className="card-hover bg-gray-50 p-6 rounded-xl">
              <div className="w-12 h-12 bg-blue-100 rounded-lg flex items-center justify-center mb-4">
                <Zap className="w-6 h-6 text-blue-600" />
              </div>
              <h3 className="text-xl font-semibold text-gray-900 mb-3">Lightning Fast</h3>
              <p className="text-gray-600">
                Process 10,000+ pages per minute with Tokio async runtime and parallel processing
              </p>
            </div>

            <div className="card-hover bg-gray-50 p-6 rounded-xl">
              <div className="w-12 h-12 bg-green-100 rounded-lg flex items-center justify-center mb-4">
                <Shield className="w-6 h-6 text-green-600" />
              </div>
              <h3 className="text-xl font-semibold text-gray-900 mb-3">Respectful Crawling</h3>
              <p className="text-gray-600">
                Built-in robots.txt compliance and politeness engine with rate limiting
              </p>
            </div>

            <div className="card-hover bg-gray-50 p-6 rounded-xl">
              <div className="w-12 h-12 bg-purple-100 rounded-lg flex items-center justify-center mb-4">
                <Network className="w-6 h-6 text-purple-600" />
              </div>
              <h3 className="text-xl font-semibold text-gray-900 mb-3">Knowledge Graphs</h3>
              <p className="text-gray-600">
                Build and query entity-relationship graphs from crawled content automatically
              </p>
            </div>

            <div className="card-hover bg-gray-50 p-6 rounded-xl">
              <div className="w-12 h-12 bg-red-100 rounded-lg flex items-center justify-center mb-4">
                <Cpu className="w-6 h-6 text-red-600" />
              </div>
              <h3 className="text-xl font-semibold text-gray-900 mb-3">Smart Processing</h3>
              <p className="text-gray-600">
                AI-powered entity recognition, content classification, and semantic analysis
              </p>
            </div>

            <div className="card-hover bg-gray-50 p-6 rounded-xl">
              <div className="w-12 h-12 bg-yellow-100 rounded-lg flex items-center justify-center mb-4">
                <Database className="w-6 h-6 text-yellow-600" />
              </div>
              <h3 className="text-xl font-semibold text-gray-900 mb-3">Multiple Storage</h3>
              <p className="text-gray-600">
                RocksDB, PostgreSQL, and graph database support with vector embeddings
              </p>
            </div>

            <div className="card-hover bg-gray-50 p-6 rounded-xl">
              <div className="w-12 h-12 bg-indigo-100 rounded-lg flex items-center justify-center mb-4">
                <Globe className="w-6 h-6 text-indigo-600" />
              </div>
              <h3 className="text-xl font-semibold text-gray-900 mb-3">Browser Support</h3>
              <p className="text-gray-600">
                Handle JavaScript-heavy sites with integrated browser automation
              </p>
            </div>
          </div>
        </div>
      </section>

      {/* Tech Stack Section */}
      <section className="py-20 bg-gray-50">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-16">
            <h2 className="text-3xl md:text-4xl font-bold text-gray-900 mb-4">
              Powered by Best-in-Class Technologies
            </h2>
            <p className="text-xl text-gray-600 max-w-2xl mx-auto">
              Built with modern Rust ecosystem and proven libraries
            </p>
          </div>

          <div className="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-6 gap-8 items-center">
            <div className="flex flex-col items-center space-y-2">
              <img 
                src="https://raw.githubusercontent.com/rust-lang/rust-artwork/master/logo/rust-logo-64x64.png"
                alt="Rust"
                className="w-12 h-12"
              />
              <span className="text-sm font-medium text-gray-600">Rust</span>
            </div>
            
            <div className="flex flex-col items-center space-y-2">
              <img 
                src="https://raw.githubusercontent.com/tokio-rs/website/master/public/img/tokio.svg"
                alt="Tokio"
                className="w-12 h-12"
              />
              <span className="text-sm font-medium text-gray-600">Tokio</span>
            </div>

            <div className="flex flex-col items-center space-y-2">
              <img 
                src="https://raw.githubusercontent.com/seanmonstar/reqwest/master/assets/reqwest.png"
                alt="Reqwest"
                className="w-12 h-12"
              />
              <span className="text-sm font-medium text-gray-600">Reqwest</span>
            </div>

            <div className="flex flex-col items-center space-y-2">
              <img 
                src="https://raw.githubusercontent.com/serde-rs/serde/master/serde-logo.png"
                alt="Serde"
                className="w-12 h-12"
              />
              <span className="text-sm font-medium text-gray-600">Serde</span>
            </div>

            <div className="flex flex-col items-center space-y-2">
              <img 
                src="https://avatars.githubusercontent.com/u/56036552?s=200&v=4"
                alt="Axum"
                className="w-12 h-12 rounded-lg"
              />
              <span className="text-sm font-medium text-gray-600">Axum</span>
            </div>

            <div className="flex flex-col items-center space-y-2">
              <img 
                src="https://github.com/clap-rs/clap/raw/master/assets/clap.png"
                alt="Clap"
                className="w-12 h-12"
              />
              <span className="text-sm font-medium text-gray-600">Clap</span>
            </div>
          </div>
        </div>
      </section>

      {/* Quick Start Section */}
      <section className="py-20 bg-white">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-16">
            <h2 className="text-3xl md:text-4xl font-bold text-gray-900 mb-4">
              Get Started in Minutes
            </h2>
            <p className="text-xl text-gray-600 max-w-2xl mx-auto">
              Install Omnivore and start crawling immediately
            </p>
          </div>

          <div className="grid md:grid-cols-3 gap-8">
            <div className="text-center">
              <div className="w-16 h-16 bg-blue-100 rounded-full flex items-center justify-center mx-auto mb-4">
                <Download className="w-8 h-8 text-blue-600" />
              </div>
              <h3 className="text-xl font-semibold text-gray-900 mb-3">1. Install</h3>
              <div className="bg-gray-900 rounded-lg p-4 text-left">
                <code className="text-green-400 font-mono text-sm">
                  # Homebrew<br />
                  brew install omnivore<br /><br />
                  # From source<br />
                  cargo install omnivore-cli
                </code>
              </div>
            </div>

            <div className="text-center">
              <div className="w-16 h-16 bg-green-100 rounded-full flex items-center justify-center mx-auto mb-4">
                <Zap className="w-8 h-8 text-green-600" />
              </div>
              <h3 className="text-xl font-semibold text-gray-900 mb-3">2. Configure</h3>
              <div className="bg-gray-900 rounded-lg p-4 text-left">
                <code className="text-green-400 font-mono text-sm">
                  # Basic crawl<br />
                  omnivore crawl https://example.com<br /><br />
                  # Advanced options<br />
                  omnivore crawl --workers 10 --depth 5
                </code>
              </div>
            </div>

            <div className="text-center">
              <div className="w-16 h-16 bg-purple-100 rounded-full flex items-center justify-center mx-auto mb-4">
                <Network className="w-8 h-8 text-purple-600" />
              </div>
              <h3 className="text-xl font-semibold text-gray-900 mb-3">3. Analyze</h3>
              <div className="bg-gray-900 rounded-lg p-4 text-left">
                <code className="text-green-400 font-mono text-sm">
                  # Build knowledge graph<br />
                  omnivore graph data.json<br /><br />
                  # Start API server<br />
                  omnivore-api
                </code>
              </div>
            </div>
          </div>

          <div className="text-center mt-12">
            <Link
              href="/docs/quickstart"
              className="inline-flex items-center px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors font-medium"
            >
              View Full Guide
              <ArrowRight className="w-5 h-5 ml-2" />
            </Link>
          </div>
        </div>
      </section>

      {/* Footer */}
      <footer className="bg-gray-900 text-white py-16">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="grid md:grid-cols-4 gap-8">
            <div>
              <div className="flex items-center space-x-2 mb-4">
                <div className="w-8 h-8 bg-gradient-to-br from-blue-500 to-purple-600 rounded-lg flex items-center justify-center">
                  <Globe className="w-5 h-5 text-white" />
                </div>
                <span className="text-xl font-bold">Omnivore</span>
              </div>
              <p className="text-gray-400 mb-4">
                Universal web crawler and knowledge graph system built in Rust.
              </p>
              <div className="flex space-x-4">
                <a href="https://github.com/yourusername/omnivore" className="text-gray-400 hover:text-white transition-colors">
                  <Github className="w-5 h-5" />
                </a>
                <a href="#" className="text-gray-400 hover:text-white transition-colors">
                  <Star className="w-5 h-5" />
                </a>
              </div>
            </div>

            <div>
              <h3 className="font-semibold mb-4">Documentation</h3>
              <ul className="space-y-2 text-gray-400">
                <li><Link href="/docs/installation" className="hover:text-white transition-colors">Installation</Link></li>
                <li><Link href="/docs/quickstart" className="hover:text-white transition-colors">Quick Start</Link></li>
                <li><Link href="/docs/configuration" className="hover:text-white transition-colors">Configuration</Link></li>
                <li><Link href="/api" className="hover:text-white transition-colors">API Reference</Link></li>
              </ul>
            </div>

            <div>
              <h3 className="font-semibold mb-4">Guides</h3>
              <ul className="space-y-2 text-gray-400">
                <li><Link href="/guides/basic-crawling" className="hover:text-white transition-colors">Basic Crawling</Link></li>
                <li><Link href="/guides/knowledge-graphs" className="hover:text-white transition-colors">Knowledge Graphs</Link></li>
                <li><Link href="/guides/performance" className="hover:text-white transition-colors">Performance Tuning</Link></li>
                <li><Link href="/guides/deployment" className="hover:text-white transition-colors">Deployment</Link></li>
              </ul>
            </div>

            <div>
              <h3 className="font-semibold mb-4">Community</h3>
              <ul className="space-y-2 text-gray-400">
                <li><a href="https://github.com/yourusername/omnivore" className="hover:text-white transition-colors">GitHub</a></li>
                <li><a href="#" className="hover:text-white transition-colors">Discord</a></li>
                <li><a href="#" className="hover:text-white transition-colors">Discussions</a></li>
                <li><a href="#" className="hover:text-white transition-colors">Contributing</a></li>
              </ul>
            </div>
          </div>

          <div className="border-t border-gray-800 mt-12 pt-8 flex flex-col md:flex-row justify-between items-center">
            <p className="text-gray-400">
              © 2024 Omnivore. Released under MIT and Apache-2.0 licenses.
            </p>
            <p className="text-gray-400 mt-4 md:mt-0">
              Built with ❤️ and Rust
            </p>
          </div>
        </div>
      </footer>
    </div>
  )
}