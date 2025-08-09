import type { Metadata } from 'next'
import { Inter, JetBrains_Mono } from 'next/font/google'
import './globals.css'
import dynamic from 'next/dynamic'

const CopyCodeClient = dynamic(() => import('@/components/CopyCodeClient'), { ssr: false })

const inter = Inter({ subsets: ['latin'] })
const jetbrainsMono = JetBrains_Mono({ 
  subsets: ['latin'],
  variable: '--font-mono'
})

export const metadata: Metadata = {
  title: 'Omnivore - Universal Web Crawler & Knowledge Graph',
  description: 'High-performance, parallel web crawler and knowledge graph system built in Rust. Extract, analyze, and graph data from the web at scale.',
  keywords: ['web crawler', 'rust', 'knowledge graph', 'data extraction', 'scraping', 'parallel processing'],
  authors: [{ name: 'Omnivore Team' }],
  openGraph: {
    title: 'Omnivore - Universal Web Crawler & Knowledge Graph',
    description: 'High-performance, parallel web crawler and knowledge graph system built in Rust.',
    type: 'website',
    images: [
      {
        url: '/og-image.png',
        width: 1200,
        height: 630,
        alt: 'Omnivore Web Crawler',
      },
    ],
  },
  twitter: {
    card: 'summary_large_image',
    title: 'Omnivore - Universal Web Crawler & Knowledge Graph',
    description: 'High-performance, parallel web crawler and knowledge graph system built in Rust.',
    images: ['/og-image.png'],
  },
  viewport: 'width=device-width, initial-scale=1',
  themeColor: '#0ea5e9',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en" className={`${inter.className} ${jetbrainsMono.variable}`}>
      <body className="min-h-screen bg-gradient-to-br from-slate-50 to-slate-100">
        {children}
        <CopyCodeClient />
      </body>
    </html>
  )
}