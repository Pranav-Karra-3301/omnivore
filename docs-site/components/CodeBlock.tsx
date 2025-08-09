"use client"

import React, { useState } from 'react'

type CodeBlockProps = {
  children: string
  language?: string
  className?: string
}

export default function CodeBlock({ children, language = 'bash', className = '' }: CodeBlockProps) {
  const [copied, setCopied] = useState(false)

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(children)
      setCopied(true)
      setTimeout(() => setCopied(false), 1500)
    } catch (error) {
      console.error('Copy failed', error)
    }
  }

  return (
    <div className={`relative group ${className}`}>
      <button
        onClick={handleCopy}
        className="absolute right-3 top-3 z-10 rounded-md border border-gray-300 bg-white/80 px-2.5 py-1 text-xs font-medium text-gray-700 shadow-sm backdrop-blur hover:bg-white focus:outline-none"
        aria-label="Copy code"
      >
        {copied ? 'Copied' : 'Copy'}
      </button>
      <pre className="rounded-lg bg-gray-900 p-4 text-sm text-gray-100 overflow-x-auto">
        <code className={`language-${language}`}>{children}</code>
      </pre>
    </div>
  )
}
