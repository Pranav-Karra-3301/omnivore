import {
  Zap,
  Globe,
  Network,
  BarChart3,
  Play,
  CheckCircle,
  ArrowRight,
} from 'lucide-react'
import CodeBlock from '@/components/CodeBlock'

export default function QuickStartPage() {
  return (
    <div className="prose prose-lg max-w-none">
      <h1 className="text-4xl font-bold text-gray-900 mb-6">Quick Start Guide</h1>

      <p className="text-xl text-gray-600 mb-8">
        Get up and running with Omnivore in just a few minutes. This guide will
        walk you through your first crawl, data extraction, and knowledge graph
        creation.
      </p>

      <div className="not-prose bg-gradient-to-r from-blue-50 to-blue-100 border border-blue-200 rounded-lg p-6 mb-8">
        <div className="flex items-center mb-3">
          <Zap className="w-6 h-6 text-blue-600 mr-2" />
          <h3 className="text-lg font-semibold text-blue-900">Prerequisites</h3>
        </div>
        <p className="text-blue-800">
          Make sure you have Omnivore installed. If not, check the
          <a href="/docs/installation" className="text-blue-600 hover:text-blue-700 ml-1">
            Installation Guide
          </a>
          first.
        </p>
      </div>

      <h2>Step 1: Your First Crawl</h2>
      <p>Let's start with a simple website crawl.</p>
      <CodeBlock language="bash">{`# Basic crawl with default settings
omnivore crawl https://httpbin.org/html`}</CodeBlock>

      <h3>Add More Options</h3>
      <CodeBlock language="bash">{`# Crawl with more workers and depth
omnivore crawl https://example.com \
  --workers 5 \
  --depth 3 \
  --delay 200 \
  --output results.json

# This will:
# - Use 5 parallel workers
# - Crawl up to 3 levels deep
# - Wait 200ms between requests
# - Save results to results.json`}</CodeBlock>

      <h3>Understanding the Output</h3>
      <div className="not-prose bg-gray-50 rounded-lg p-4 my-6">
        <h4 className="font-medium text-gray-900 mb-3">Output Explained</h4>
        <div className="space-y-2 text-sm">
          <div className="flex items-center">
            <span className="w-24 text-gray-600">Crawled:</span>
            <span className="text-gray-900">Total URLs discovered and processed</span>
          </div>
          <div className="flex items-center">
            <span className="w-24 text-gray-600">Success:</span>
            <span className="text-gray-900">URLs successfully fetched and parsed</span>
          </div>
          <div className="flex items-center">
            <span className="w-24 text-gray-600">Failed:</span>
            <span className="text-gray-900">URLs that couldn't be fetched</span>
          </div>
          <div className="flex items-center">
            <span className="w-24 text-gray-600">In Progress:</span>
            <span className="text-gray-900">URLs currently being processed</span>
          </div>
        </div>
      </div>

      <h2>Step 2: Parse Specific Content</h2>
      <p>Extract specific data from HTML using custom parsing rules.</p>
      <h3>Create a Parser Configuration</h3>
      <CodeBlock language="bash">{`# Create a parser config file
cat > parser-rules.yaml << EOF
rules:
  - name: "title"
    selector: "title"
    required: true
  - name: "headings"
    selector: "h1, h2, h3"
    multiple: true
  - name: "links"
    selector: "a[href]"
    attribute: "href"
    multiple: true
EOF`}</CodeBlock>

      <h3>Parse with Custom Rules</h3>
      <CodeBlock language="bash">{`# Parse a specific HTML file
omnivore parse index.html --rules parser-rules.yaml

# Parse from URL and extract data
curl -s https://example.com | omnivore parse --rules parser-rules.yaml`}</CodeBlock>

      <h2>Step 3: Build Knowledge Graphs</h2>
      <p>Transform your crawled data into a knowledge graph for deeper analysis.</p>
      <CodeBlock language="bash">{`# Build a knowledge graph from crawl results
omnivore graph results.json --output knowledge-graph.db

# Query the knowledge graph
omnivore graph-query knowledge-graph.db \
  --query "MATCH (n:Website)-[r:LINKS_TO]->(m:Website) RETURN n, r, m LIMIT 10"`}</CodeBlock>

      <h2>Common Use Cases</h2>
      <h3>1. News Site Monitoring</h3>
      <CodeBlock language="bash">{`# Monitor a news website for new articles
omnivore crawl https://news.ycombinator.com \
  --workers 3 \
  --depth 2 \
  --respect-robots \
  --output hn-$(date +%Y%m%d).json

# Extract article data with custom rules
omnivore parse hn-*.json --rules news-extractor.yaml`}</CodeBlock>

      <h3>2. E-commerce Product Scraping</h3>
      <CodeBlock language="bash">{`# Scrape product information (respect rate limits!)
omnivore crawl https://example-store.com/products \
  --workers 2 \
  --delay 1000 \
  --depth 3 \
  --output products.json

# Parse product data
omnivore parse products.json --rules product-extractor.yaml`}</CodeBlock>

      <h3>3. Research Paper Collection</h3>
      <CodeBlock language="bash">{`# Collect academic papers
omnivore crawl https://arxiv.org/list/cs.AI/recent \
  --workers 5 \
  --depth 2 \
  --output papers.json

# Build citation network
omnivore graph papers.json \
  --schema academic-papers \
  --output citation-network.db`}</CodeBlock>

      <h2>Configuration Examples</h2>
      <h3>Basic Configuration File</h3>
      <p>
        Create <code>~/.config/omnivore/crawler.toml</code>:
      </p>
      <CodeBlock language="toml">{`[crawler]
max_workers = 10
max_depth = 5
user_agent = "Omnivore/1.0 (+https://yoursite.com/bot)"
respect_robots_txt = true
timeout_ms = 30000

[crawler.politeness]
default_delay_ms = 100
max_requests_per_second = 10.0
backoff_multiplier = 2.0

[parser]
clean_text = true
extract_metadata = true

[storage]
data_dir = "~/.local/share/omnivore"
cache_size_mb = 1024
compression = "zstd"`}</CodeBlock>

      <h3>Domain-Specific Configuration</h3>
      <CodeBlock language="toml">{`# Site-specific settings
[[crawler.site_configs]]
domain = "example.com"
delay_ms = 500
max_requests_per_second = 2.0
custom_headers = { "X-API-Key" = "your-key" }

[[crawler.site_configs]]
domain = "news.ycombinator.com"
delay_ms = 1000
max_depth = 3`}</CodeBlock>

      <h2>Monitoring and Statistics</h2>
      <h3>View Crawl Statistics</h3>
      <CodeBlock language="bash">{`# Current session stats
omnivore stats

# Historical stats
omnivore stats --session crawl-20240809

# Export stats to CSV
omnivore stats --export stats.csv`}</CodeBlock>

      <h3>Real-time Monitoring</h3>
      <CodeBlock language="bash">{`# Watch crawl progress
omnivore crawl https://example.com --watch`}</CodeBlock>

      <h2>Tips for Success</h2>
      <div className="not-prose bg-yellow-50 border border-yellow-200 rounded-lg p-6 my-8">
        <h3 className="text-lg font-semibold text-yellow-900 mb-4">💡 Pro Tips</h3>
        <ul className="space-y-3 text-yellow-800">
          <li className="flex items-start">
            <CheckCircle className="w-5 h-5 text-yellow-600 mr-2 mt-0.5 flex-shrink-0" />
            <span>
              <strong>Start small:</strong> Begin with low depth and workers, then scale up
            </span>
          </li>
          <li className="flex items-start">
            <CheckCircle className="w-5 h-5 text-yellow-600 mr-2 mt-0.5 flex-shrink-0" />
            <span>
              <strong>Respect robots.txt:</strong> Always enable robots.txt compliance
            </span>
          </li>
          <li className="flex items-start">
            <CheckCircle className="w-5 h-5 text-yellow-600 mr-2 mt-0.5 flex-shrink-0" />
            <span>
              <strong>Monitor resources:</strong> Watch CPU and memory usage during crawls
            </span>
          </li>
          <li className="flex items-start">
            <CheckCircle className="w-5 h-5 text-yellow-600 mr-2 mt-0.5 flex-shrink-0" />
            <span>
              <strong>Use delays:</strong> Be respectful with request timing
            </span>
          </li>
          <li className="flex items-start">
            <CheckCircle className="w-5 h-5 text-yellow-600 mr-2 mt-0.5 flex-shrink-0" />
            <span>
              <strong>Save configurations:</strong> Create reusable config files for common patterns
            </span>
          </li>
        </ul>
      </div>

      <p>
        Ready to dive deeper? Check out the{' '}
        <a href="/docs/configuration" className="text-blue-600 hover:text-blue-700">
          Configuration Guide
        </a>{' '}
        to customize Omnivore for your specific needs.
      </p>
    </div>
  )
}
