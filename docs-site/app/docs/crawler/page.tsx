export default function CrawlerPage() {
  return (
    <div className="prose prose-lg max-w-none">
      <h1 className="text-4xl font-bold text-gray-900 mb-6">Crawler Engine</h1>
      <p className="text-xl text-gray-600 mb-8">Overview of the parallel crawler, frontier, politeness engine, and workers.</p>

      <h2>Architecture</h2>
      <ul>
        <li>Frontier: manages URLs and depths</li>
        <li>Scheduler: executes tasks with a worker pool</li>
        <li>Politeness Engine: robots.txt and per-domain pacing</li>
        <li>Workers: fetch, parse, extract links</li>
      </ul>

      <h2>Best Practices</h2>
      <ul>
        <li>Start with low depth and workers</li>
        <li>Respect robots.txt</li>
        <li>Add delays for sensitive domains</li>
      </ul>
    </div>
  )
}
