'use client'
import Link from 'next/link'
import { usePathname } from 'next/navigation'
import { 
  Book, 
  Download, 
  Settings, 
  Zap,
  Globe,
  Network,
  Code,
  ArrowLeft
} from 'lucide-react'

const navigation = [
  {
    name: 'Getting Started',
    items: [
      { name: 'Installation', href: '/docs/installation', icon: Download },
      { name: 'Quick Start', href: '/docs/quickstart', icon: Zap },
      { name: 'Configuration', href: '/docs/configuration', icon: Settings },
    ]
  },
  {
    name: 'Core Concepts',
    items: [
      { name: 'Crawler Engine', href: '/docs/crawler', icon: Globe },
      { name: 'Knowledge Graphs', href: '/docs/knowledge-graphs', icon: Network },
      { name: 'Data Processing', href: '/docs/processing', icon: Code },
    ]
  },
  {
    name: 'API Reference',
    items: [
      { name: 'CLI Commands', href: '/docs/cli', icon: Code },
      { name: 'REST API', href: '/docs/rest-api', icon: Code },
      { name: 'GraphQL API', href: '/docs/graphql-api', icon: Code },
    ]
  },
]

export default function DocsLayout({
  children,
}: {
  children: React.ReactNode
}) {
  const pathname = usePathname()

  return (
    <div className="min-h-screen bg-white">
      {/* Header */}
      <header className="bg-white border-b border-gray-200 sticky top-0 z-40">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center h-16">
            <div className="flex items-center space-x-4">
              <Link href="/" className="flex items-center space-x-2 text-gray-600 hover:text-gray-900 transition-colors">
                <ArrowLeft className="w-4 h-4" />
                <span>Back to Home</span>
              </Link>
              <span className="text-gray-300">|</span>
              <div className="flex items-center space-x-2">
                <div className="w-6 h-6 bg-gradient-to-br from-blue-500 to-purple-600 rounded-md flex items-center justify-center">
                  <Globe className="w-4 h-4 text-white" />
                </div>
                <span className="font-semibold text-gray-900">Omnivore Docs</span>
              </div>
            </div>
          </div>
        </div>
      </header>

      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex">
          {/* Sidebar */}
          <nav className="w-64 flex-shrink-0 py-8 pr-8">
            <div className="space-y-8">
              {navigation.map((section) => (
                <div key={section.name}>
                  <h3 className="text-sm font-semibold text-gray-900 uppercase tracking-wider mb-3">
                    {section.name}
                  </h3>
                  <ul className="space-y-1">
                    {section.items.map((item) => {
                      const Icon = item.icon
                      const isActive = pathname === item.href
                      return (
                        <li key={item.name}>
                          <Link
                            href={item.href}
                            className={`flex items-center space-x-3 px-3 py-2 rounded-lg text-sm transition-colors ${
                              isActive
                                ? 'bg-blue-50 text-blue-700 border-r-2 border-blue-500'
                                : 'text-gray-600 hover:text-gray-900 hover:bg-gray-50'
                            }`}
                          >
                            <Icon className="w-4 h-4" />
                            <span>{item.name}</span>
                          </Link>
                        </li>
                      )
                    })}
                  </ul>
                </div>
              ))}
            </div>
          </nav>

          {/* Main content */}
          <main className="flex-1 py-8 pl-8">
            <div className="max-w-4xl">
              {children}
            </div>
          </main>
        </div>
      </div>
    </div>
  )
}