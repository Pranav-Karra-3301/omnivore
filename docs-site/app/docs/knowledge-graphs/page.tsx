export default function KnowledgeGraphsPage() {
  return (
    <div className="prose prose-lg max-w-none">
      <h1 className="text-4xl font-bold text-gray-900 mb-6">Knowledge Graphs</h1>
      <p className="text-xl text-gray-600 mb-8">Turn crawled data into an entity-relationship graph for deeper insights.</p>

      <h2>Concepts</h2>
      <ul>
        <li>Entities (nodes) and Relationships (edges)</li>
        <li>Extraction via parsing rules</li>
        <li>Export to a graph DB or file</li>
      </ul>
    </div>
  )
}
