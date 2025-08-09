import Link from 'next/link'
import CodeBlock from '@/components/CodeBlock'
import { 
  Download, 
  Zap, 
  Settings, 
  Globe, 
  Network, 
  Code, 
  ArrowRight,
  Book,
  ExternalLink
} from 'lucide-react'

export default function DocsPage() {
  return (
    <div className="prose prose-lg max-w-none">
      <h1 className="text-4xl font-bold text-gray-900 mb-6">
        Omnivore Documentation
      </h1>
      
      <p className="text-xl text-gray-600 mb-8">
        Welcome to the comprehensive documentation for Omnivore, the Universal Rust Web Crawler & Knowledge Graph Builder.
        Get started quickly or dive deep into advanced features.
      </p>

      {/* Quick Navigation Cards */}
      <div className="not-prose grid md:grid-cols-2 lg:grid-cols-3 gap-6 my-12">
        <Link href="/docs/installation" className="group block p-6 bg-gradient-to-br from-blue-50 to-blue-100 rounded-xl border border-blue-200 hover:border-blue-300 transition-all">
          <div className="flex items-center space-x-3 mb-3">
            <div className="w-10 h-10 bg-blue-500 rounded-lg flex items-center justify-center">
              <Download className="w-5 h-5 text-white" />
            </div>
            <h3 className="text-lg font-semibold text-gray-900">Installation</h3>
          </div>
          <p className="text-gray-600 text-sm mb-3">
            Get Omnivore installed on your system with Homebrew, Docker, or from source.
          </p>
          <div className="flex items-center text-blue-600 text-sm font-medium group-hover:text-blue-700">
            Start Installing <ArrowRight className="w-4 h-4 ml-1" />
          </div>
        </Link>

        <Link href="/docs/quickstart" className="group block p-6 bg-gradient-to-br from-green-50 to-green-100 rounded-xl border border-green-200 hover:border-green-300 transition-all">
          <div className="flex items-center space-x-3 mb-3">
            <div className="w-10 h-10 bg-green-500 rounded-lg flex items-center justify-center">
              <Zap className="w-5 h-5 text-white" />
            </div>
            <h3 className="text-lg font-semibold text-gray-900">Quick Start</h3>
          </div>
          <p className="text-gray-600 text-sm mb-3">
            Start crawling websites in minutes with simple commands and examples.
          </p>
          <div className="flex items-center text-green-600 text-sm font-medium group-hover:text-green-700">
            Get Started <ArrowRight className="w-4 h-4 ml-1" />
          </div>
        </Link>

        <Link href="/docs/configuration" className="group block p-6 bg-gradient-to-br from-purple-50 to-purple-100 rounded-xl border border-purple-200 hover:border-purple-300 transition-all">
          <div className="flex items-center space-x-3 mb-3">
            <div className="w-10 h-10 bg-purple-500 rounded-lg flex items-center justify-center">
              <Settings className="w-5 h-5 text-white" />
            </div>
            <h3 className="text-lg font-semibold text-gray-900">Configuration</h3>
          </div>
          <p className="text-gray-600 text-sm mb-3">
            Configure Omnivore for your specific crawling needs and use cases.
          </p>
          <div className="flex items-center text-purple-600 text-sm font-medium group-hover:text-purple-700">
            Learn More <ArrowRight className="w-4 h-4 ml-1" />
          </div>
        </Link>

        <Link href="/docs/crawler" className="group block p-6 bg-gradient-to-br from-orange-50 to-orange-100 rounded-xl border border-orange-200 hover:border-orange-300 transition-all">
          <div className="flex items-center space-x-3 mb-3">
            <div className="w-10 h-10 bg-orange-500 rounded-lg flex items-center justify-center">
              <Globe className="w-5 h-5 text-white" />
            </div>
            <h3 className="text-lg font-semibold text-gray-900">Crawler Engine</h3>
          </div>
          <p className="text-gray-600 text-sm mb-3">
            Understand how Omnivore's parallel crawler engine works under the hood.
          </p>
          <div className="flex items-center text-orange-600 text-sm font-medium group-hover:text-orange-700">
            Deep Dive <ArrowRight className="w-4 h-4 ml-1" />
          </div>
        </Link>

        <Link href="/docs/knowledge-graphs" className="group block p-6 bg-gradient-to-br from-indigo-50 to-indigo-100 rounded-xl border border-indigo-200 hover:border-indigo-300 transition-all">
          <div className="flex items-center space-x-3 mb-3">
            <div className="w-10 h-10 bg-indigo-500 rounded-lg flex items-center justify-center">
              <Network className="w-5 h-5 text-white" />
            </div>
            <h3 className="text-lg font-semibold text-gray-900">Knowledge Graphs</h3>
          </div>
          <p className="text-gray-600 text-sm mb-3">
            Build and query knowledge graphs from your crawled data.
          </p>
          <div className="flex items-center text-indigo-600 text-sm font-medium group-hover:text-indigo-700">
            Explore Graphs <ArrowRight className="w-4 h-4 ml-1" />
          </div>
        </Link>

        <Link href="/docs/cli" className="group block p-6 bg-gradient-to-br from-red-50 to-red-100 rounded-xl border border-red-200 hover:border-red-300 transition-all">
          <div className="flex items-center space-x-3 mb-3">
            <div className="w-10 h-10 bg-red-500 rounded-lg flex items-center justify-center">
              <Code className="w-5 h-5 text-white" />
            </div>
            <h3 className="text-lg font-semibold text-gray-900">CLI Reference</h3>
          </div>
          <p className="text-gray-600 text-sm mb-3">
            Complete reference for all CLI commands and options.
          </p>
          <div className="flex items-center text-red-600 text-sm font-medium group-hover:text-red-700">
            View Commands <ArrowRight className="w-4 h-4 ml-1" />
          </div>
        </Link>
      </div>

      <h2>What's Inside</h2>
      <p>This documentation covers everything you need to know about Omnivore:</p>

      <h3>Core Features</h3>
      <ul>
        <li><strong>Parallel Crawling</strong>: Async/await with Tokio runtime for maximum performance</li>
        <li><strong>Smart Processing</strong>: AI-powered entity recognition and content classification</li>
        <li><strong>Knowledge Graphs</strong>: Build entity-relationship graphs automatically</li>
        <li><strong>Respectful Crawling</strong>: Built-in robots.txt compliance and rate limiting</li>
        <li><strong>Multiple Storage</strong>: Support for RocksDB, PostgreSQL, and graph databases</li>
      </ul>

      <h3>Architecture</h3>
      <ul>
        <li><strong>omnivore-core</strong>: Core crawler and processing engine</li>
        <li><strong>omnivore-cli</strong>: Command-line interface with rich features</li>
        <li><strong>Plugin System</strong>: Extensible architecture for custom processors</li>
      </ul>

      <h3>Deployment Options</h3>
      <ul>
        <li><strong>Homebrew</strong>: Easy installation on macOS and Linux</li>
        <li><strong>Docker</strong>: Containerized deployment with orchestration</li>
        <li><strong>From Source</strong>: Build and customize for your needs</li>
      </ul>

      <h2>Getting Help</h2>

      <div className="not-prose bg-gray-50 rounded-lg p-6 my-8">
        <h3 className="text-lg font-semibold text-gray-900 mb-4 flex items-center">
          <Book className="w-5 h-5 mr-2" />
          Need Help?
        </h3>
        <div className="grid md:grid-cols-2 gap-4">
          <div>
            <h4 className="font-medium text-gray-900 mb-2">Community Resources</h4>
            <ul className="space-y-2 text-sm">
              <li>
                <a href="https://github.com/Pranav-Karra-3301/omnivore/issues" className="text-blue-600 hover:text-blue-700 flex items-center">
                  GitHub Issues <ExternalLink className="w-3 h-3 ml-1" />
                </a>
              </li>
              <li>
                <a href="https://github.com/Pranav-Karra-3301/omnivore/discussions" className="text-blue-600 hover:text-blue-700 flex items-center">
                  Discussions <ExternalLink className="w-3 h-3 ml-1" />
                </a>
              </li>
              <li>
                <Link href="/guides/troubleshooting" className="text-blue-600 hover:text-blue-700">
                  Troubleshooting Guide
                </Link>
              </li>
            </ul>
          </div>
          <div>
            <h4 className="font-medium text-gray-900 mb-2">Documentation</h4>
            <ul className="space-y-2 text-sm">
              <li>
                <Link href="/guides/faq" className="text-blue-600 hover:text-blue-700">
                  Frequently Asked Questions
                </Link>
              </li>
              <li>
                <Link href="/examples" className="text-blue-600 hover:text-blue-700">
                  Code Examples
                </Link>
              </li>
              <li>
                <Link href="/guides/contributing" className="text-blue-600 hover:text-blue-700">
                  Contributing Guide
                </Link>
              </li>
            </ul>
          </div>
        </div>
      </div>

      <h2>Quick Example</h2>
      <p>Here's a simple example to get you started:</p>

      <CodeBlock language="bash">{`# Install Omnivore
brew install omnivore

# Start crawling
omnivore crawl https://example.com --workers 5 --depth 3

# Build knowledge graph
omnivore graph crawl-results.json --output knowledge-graph.db`}</CodeBlock>

      <h2>What's Next?</h2>
      <ul>
        <li><strong>New to Omnivore?</strong> Start with the <Link href="/docs/installation" className="text-blue-600 hover:text-blue-700">Installation Guide</Link></li>
        <li><strong>Ready to crawl?</strong> Jump to <Link href="/docs/quickstart" className="text-blue-600 hover:text-blue-700">Quick Start</Link></li>
        <li><strong>Want to customize?</strong> Check out <Link href="/docs/configuration" className="text-blue-600 hover:text-blue-700">Configuration</Link></li>
        <li><strong>Building integrations?</strong> Explore the CLI and data formats in the docs</li>
      </ul>

      <div className="not-prose mt-8 text-sm text-gray-500">
        More documentation references are available on <a href="https://readthedocs.org" className="text-blue-600 hover:text-blue-700" target="_blank" rel="noopener noreferrer">readthedocs.org</a>.
      </div>
    </div>
  )
}