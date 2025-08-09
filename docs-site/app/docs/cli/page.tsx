import CodeBlock from '@/components/CodeBlock'

export default function CLIPage() {
  return (
    <div className="prose prose-lg max-w-none">
      <h1 className="text-4xl font-bold text-gray-900 mb-6">CLI Reference</h1>
      <p className="text-xl text-gray-600 mb-8">Key commands available in the Omnivore CLI.</p>

      <h2>Commands</h2>
      <h3>crawl</h3>
      <CodeBlock language="bash">{`omnivore crawl https://example.com --workers 5 --depth 3`}</CodeBlock>

      <h3>parse</h3>
      <CodeBlock language="bash">{`omnivore parse index.html --rules parser-rules.yaml`}</CodeBlock>

      <h3>graph</h3>
      <CodeBlock language="bash">{`omnivore graph results.json --output knowledge-graph.db`}</CodeBlock>
    </div>
  )
}
