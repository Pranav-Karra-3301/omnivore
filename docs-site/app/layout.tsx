import type { Metadata, Viewport } from 'next'
import { Inter, JetBrains_Mono } from 'next/font/google'
import './globals.css'
import dynamic from 'next/dynamic'
import ThemeToggle from '@/components/ThemeToggle'

const CopyCodeClient = dynamic(() => import('@/components/CopyCodeClient'), { ssr: false })

const inter = Inter({ subsets: ['latin'] })
const jetbrainsMono = JetBrains_Mono({ 
  subsets: ['latin'],
  variable: '--font-mono'
})

export const metadata: Metadata = {
  metadataBase: new URL('https://omnivore.pranavkarra.me'),
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
        url: '/preview.png',
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
    images: ['/preview.png'],
  },
}

export const viewport: Viewport = {
  width: 'device-width',
  initialScale: 1,
  themeColor: '#0ea5e9',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  // Simple dark mode toggle (client hydrated via CopyCodeClient already)
  return (
    <html lang="en" className={`${inter.className} ${jetbrainsMono.variable}`}>
      <body className="min-h-screen bg-gradient-to-br from-slate-50 to-slate-100 dark:from-slate-900 dark:to-slate-950 dark:text-gray-100 transition-colors">
        <div className="fixed right-4 top-4 z-50">
          <ThemeToggle />
        </div>
        {children}
        <CopyCodeClient />
        <script dangerouslySetInnerHTML={{ __html: `(() => {try {const t=localStorage.getItem('theme'); if (t==='dark') document.documentElement.classList.add('dark');} catch{} })();` }} />
      </body>
    </html>
  )
}