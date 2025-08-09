"use client"

import { useEffect, useState } from 'react'

export default function ThemeToggle() {
  const [isDark, setIsDark] = useState<boolean>(false)

  useEffect(() => {
    try {
      const t = localStorage.getItem('theme')
      const dark = t === 'dark' || (!t && window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches)
      if (dark) document.documentElement.classList.add('dark')
      setIsDark(document.documentElement.classList.contains('dark'))
    } catch {
      // ignore
    }
  }, [])

  const toggle = () => {
    const root = document.documentElement
    const next = root.classList.contains('dark') ? 'light' : 'dark'
    root.classList.toggle('dark')
    setIsDark(next === 'dark')
    try { localStorage.setItem('theme', next) } catch {}
  }

  return (
    <button
      onClick={toggle}
      className="rounded-md border border-gray-300/60 dark:border-gray-700 bg-white/80 dark:bg-gray-800/80 px-3 py-1.5 text-sm font-medium text-gray-700 dark:text-gray-100 shadow-sm backdrop-blur hover:bg-white dark:hover:bg-gray-800"
      aria-label="Toggle dark mode"
    >
      {isDark ? 'Light mode' : 'Dark mode'}
    </button>
  )
}
