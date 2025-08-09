import { 
  Zap, 
  Globe, 
  Network, 
  BarChart3,
  Play,
  CheckCircle,
  ArrowRight
} from 'lucide-react'

export default function QuickStartPage() {
  return (
    <div className="prose prose-lg max-w-none">
      <h1 className="text-4xl font-bold text-gray-900 mb-6">
        Quick Start Guide
      </h1>
      
      <p className="text-xl text-gray-600 mb-8">
        Get up and running with Omnivore in just a few minutes. This guide will walk you through 
        your first crawl, data extraction, and knowledge graph creation.
      </p>

      <div className="not-prose bg-gradient-to-r from-blue-50 to-blue-100 border border-blue-200 rounded-lg p-6 mb-8">
        <div className="flex items-center mb-3">
          <Zap className="w-6 h-6 text-blue-600 mr-2" />
          <h3 className="text-lg font-semibold text-blue-900">Prerequisites</h3>
        </div>
        <p className="text-blue-800">
          Make sure you have Omnivore installed. If not, check the 
          <a href="/docs/installation" className="text-blue-600 hover:text-blue-700 ml-1">Installation Guide</a> first.
        </p>
      </div>

      ## Step 1: Your First Crawl

      Let's start with a simple website crawl. We'll use a test website to demonstrate the basic functionality.

      ```bash
      # Basic crawl with default settings
      omnivore crawl https://httpbin.org/html

      # The output will show:
      # 🕸️  Omnivore Web Crawler
      # Starting crawl from: https://httpbin.org/html
      # ✨ [00:00:03] Crawled: 1 | Success: 1 | Failed: 0
      # 📊 Final Statistics: 1 URLs crawled in 3s
      ```

      ### Add More Options

      ```bash
      # Crawl with more workers and depth
      omnivore crawl https://example.com \\
        --workers 5 \\
        --depth 3 \\
        --delay 200 \\
        --output results.json

      # This will:
      # - Use 5 parallel workers
      # - Crawl up to 3 levels deep
      # - Wait 200ms between requests
      # - Save results to results.json
      ```

      ### Understanding the Output

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
            <span className="text-gray-900">URLs that couldn't be fetched (404, timeout, etc.)</span>
          </div>
          <div className="flex items-center">
            <span className="w-24 text-gray-600">In Progress:</span>
            <span className="text-gray-900">URLs currently being processed</span>
          </div>
        </div>
      </div>

      ## Step 2: Parse Specific Content

      Extract specific data from HTML using custom parsing rules.

      ### Create a Parser Configuration

      ```bash
      # Create a parser config file
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
      EOF
      ```

      ### Parse with Custom Rules

      ```bash
      # Parse a specific HTML file
      omnivore parse index.html --rules parser-rules.yaml

      # Parse from URL and extract data
      curl -s https://example.com | omnivore parse --rules parser-rules.yaml
      ```

      ## Step 3: Build Knowledge Graphs

      Transform your crawled data into a knowledge graph for deeper analysis.

      ```bash
      # Build a knowledge graph from crawl results
      omnivore graph results.json --output knowledge-graph.db

      # Query the knowledge graph
      omnivore graph-query knowledge-graph.db \\
        --query "MATCH (n:Website)-[r:LINKS_TO]->(m:Website) RETURN n, r, m LIMIT 10"
      ```

      ## Step 4: Start the API Server

      Launch the REST and GraphQL API server for programmatic access.

      ```bash
      # Start the API server
      omnivore-api

      # Server starts at http://localhost:3000
      # GraphQL playground: http://localhost:3000/graphql
      # API documentation: http://localhost:3000/docs
      ```

      ### Test the API

      ```bash
      # Health check
      curl http://localhost:3000/health

      # Start a crawl via API
      curl -X POST http://localhost:3000/api/crawl \\
        -H "Content-Type: application/json" \\
        -d '{"url": "https://example.com", "max_depth": 2}'

      # Check crawl statistics
      curl http://localhost:3000/api/stats
      ```

      ## Common Use Cases

      ### 1. News Site Monitoring

      ```bash
      # Monitor a news website for new articles
      omnivore crawl https://news.ycombinator.com \\
        --workers 3 \\
        --depth 2 \\
        --respect-robots \\
        --output hn-$(date +%Y%m%d).json

      # Extract article data with custom rules
      omnivore parse hn-*.json --rules news-extractor.yaml
      ```

      ### 2. E-commerce Product Scraping

      ```bash
      # Scrape product information (respect rate limits!)
      omnivore crawl https://example-store.com/products \\
        --workers 2 \\
        --delay 1000 \\
        --depth 3 \\
        --output products.json

      # Parse product data
      omnivore parse products.json --rules product-extractor.yaml
      ```

      ### 3. Research Paper Collection

      ```bash
      # Collect academic papers
      omnivore crawl https://arxiv.org/list/cs.AI/recent \\
        --workers 5 \\
        --depth 2 \\
        --output papers.json

      # Build citation network
      omnivore graph papers.json \\
        --schema academic-papers \\
        --output citation-network.db
      ```

      ## Configuration Examples

      ### Basic Configuration File

      Create `~/.config/omnivore/crawler.toml`:

      ```toml
      [crawler]
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
      compression = "zstd"
      ```

      ### Domain-Specific Configuration

      ```toml
      # Site-specific settings
      [[crawler.site_configs]]
      domain = "example.com"
      delay_ms = 500
      max_requests_per_second = 2.0
      custom_headers = { "X-API-Key" = "your-key" }

      [[crawler.site_configs]]
      domain = "news.ycombinator.com"  
      delay_ms = 1000
      max_depth = 3
      ```

      ## Monitoring and Statistics

      ### View Crawl Statistics

      ```bash
      # Current session stats
      omnivore stats

      # Historical stats
      omnivore stats --session crawl-20240809

      # Export stats to CSV
      omnivore stats --export stats.csv
      ```

      ### Real-time Monitoring

      ```bash
      # Watch crawl progress
      omnivore crawl https://example.com --watch

      # Monitor API server
      curl http://localhost:3000/metrics  # Prometheus metrics
      ```

      ## Next Steps

      <div className="not-prose grid md:grid-cols-2 gap-6 my-8">
        <div className="bg-gradient-to-br from-blue-50 to-blue-100 rounded-xl p-6 border border-blue-200">
          <div className="w-10 h-10 bg-blue-500 rounded-lg flex items-center justify-center mb-4">
            <Globe className="w-5 h-5 text-white" />
          </div>
          <h3 className="text-lg font-semibold text-gray-900 mb-2">Learn More About Crawling</h3>
          <p className="text-gray-600 text-sm mb-4">
            Deep dive into crawler configuration, advanced patterns, and optimization techniques.
          </p>
          <a href="/docs/crawler" className="inline-flex items-center text-blue-600 hover:text-blue-700 font-medium">
            Crawler Guide <ArrowRight className="w-4 h-4 ml-1" />
          </a>
        </div>

        <div className="bg-gradient-to-br from-green-50 to-green-100 rounded-xl p-6 border border-green-200">
          <div className="w-10 h-10 bg-green-500 rounded-lg flex items-center justify-center mb-4">
            <Network className="w-5 h-5 text-white" />
          </div>
          <h3 className="text-lg font-semibold text-gray-900 mb-2">Knowledge Graphs</h3>
          <p className="text-gray-600 text-sm mb-4">
            Build and query knowledge graphs from your crawled data with advanced algorithms.
          </p>
          <a href="/docs/knowledge-graphs" className="inline-flex items-center text-green-600 hover:text-green-700 font-medium">
            Graph Guide <ArrowRight className="w-4 h-4 ml-1" />
          </a>
        </div>

        <div className="bg-gradient-to-br from-purple-50 to-purple-100 rounded-xl p-6 border border-purple-200">
          <div className="w-10 h-10 bg-purple-500 rounded-lg flex items-center justify-center mb-4">
            <BarChart3 className="w-5 h-5 text-white" />
          </div>
          <h3 className="text-lg font-semibold text-gray-900 mb-2">API Integration</h3>
          <p className="text-gray-600 text-sm mb-4">
            Integrate Omnivore into your applications using REST and GraphQL APIs.
          </p>
          <a href="/docs/rest-api" className="inline-flex items-center text-purple-600 hover:text-purple-700 font-medium">
            API Reference <ArrowRight className="w-4 h-4 ml-1" />
          </a>
        </div>

        <div className="bg-gradient-to-br from-orange-50 to-orange-100 rounded-xl p-6 border border-orange-200">
          <div className="w-10 h-10 bg-orange-500 rounded-lg flex items-center justify-center mb-4">
            <Play className="w-5 h-5 text-white" />
          </div>
          <h3 className="text-lg font-semibold text-gray-900 mb-2">Real Examples</h3>
          <p className="text-gray-600 text-sm mb-4">
            See complete examples and tutorials for common use cases and integrations.
          </p>
          <a href="/examples" className="inline-flex items-center text-orange-600 hover:text-orange-700 font-medium">
            View Examples <ArrowRight className="w-4 h-4 ml-1" />
          </a>
        </div>
      </div>

      ## Tips for Success

      <div className="not-prose bg-yellow-50 border border-yellow-200 rounded-lg p-6 my-8">
        <h3 className="text-lg font-semibold text-yellow-900 mb-4">💡 Pro Tips</h3>
        <ul className="space-y-3 text-yellow-800">
          <li className="flex items-start">
            <CheckCircle className="w-5 h-5 text-yellow-600 mr-2 mt-0.5 flex-shrink-0" />
            <span><strong>Start small:</strong> Begin with low depth and workers, then scale up</span>
          </li>
          <li className="flex items-start">
            <CheckCircle className="w-5 h-5 text-yellow-600 mr-2 mt-0.5 flex-shrink-0" />
            <span><strong>Respect robots.txt:</strong> Always enable robots.txt compliance</span>
          </li>
          <li className="flex items-start">
            <CheckCircle className="w-5 h-5 text-yellow-600 mr-2 mt-0.5 flex-shrink-0" />
            <span><strong>Monitor resources:</strong> Watch CPU and memory usage during crawls</span>
          </li>
          <li className="flex items-start">
            <CheckCircle className="w-5 h-5 text-yellow-600 mr-2 mt-0.5 flex-shrink-0" />
            <span><strong>Use delays:</strong> Be respectful with request timing</span>
          </li>
          <li className="flex items-start">
            <CheckCircle className="w-5 h-5 text-yellow-600 mr-2 mt-0.5 flex-shrink-0" />
            <span><strong>Save configurations:</strong> Create reusable config files for common patterns</span>
          </li>
        </ul>
      </div>

      ## Troubleshooting

      **Crawl seems stuck?**
      - Check if the site blocks crawlers
      - Increase delay between requests
      - Verify robots.txt compliance

      **High memory usage?**
      - Reduce number of workers
      - Lower the crawl depth
      - Enable data compression

      **Getting 403/429 errors?**
      - Add longer delays between requests
      - Use a more descriptive User-Agent
      - Check if authentication is required

      For more help, see the [Troubleshooting Guide](/guides/troubleshooting) or [FAQ](/guides/faq).

      ---

      **Ready to dive deeper?** Check out the [Configuration Guide](/docs/configuration) to customize Omnivore for your specific needs.
    </div>
  )
}