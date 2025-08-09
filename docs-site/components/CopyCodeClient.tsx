"use client"

import { useEffect } from 'react'

export default function CopyCodeClient() {
  useEffect(() => {
    const addButtons = () => {
      const codeBlocks = document.querySelectorAll('pre');
      codeBlocks.forEach((pre) => {
        const parent = pre as HTMLElement;
        if (parent.dataset.copyEnhanced === 'true') return;
        parent.dataset.copyEnhanced = 'true';
        parent.style.position = parent.style.position || 'relative';

        const button = document.createElement('button');
        button.textContent = 'Copy';
        button.setAttribute('aria-label', 'Copy code');
        button.className = 'absolute right-3 top-3 z-10 rounded-md border border-gray-300 bg-white/80 px-2.5 py-1 text-xs font-medium text-gray-700 shadow-sm backdrop-blur hover:bg-white focus:outline-none';

        button.addEventListener('click', async () => {
          const code = parent.querySelector('code');
          if (!code) return;
          const text = code.textContent || '';
          try {
            await navigator.clipboard.writeText(text);
            const original = button.textContent;
            button.textContent = 'Copied';
            setTimeout(() => {
              button.textContent = original;
            }, 1200);
          } catch (e) {
            // ignore
          }
        });

        parent.appendChild(button);
      });
    };

    addButtons();
    const observer = new MutationObserver(addButtons);
    observer.observe(document.body, { childList: true, subtree: true });
    return () => observer.disconnect();
  }, []);

  return null;
}
