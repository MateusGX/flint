import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'Flint',
  description: 'A small register-based language and HTTP runtime for APIs and server-rendered pages',
  lang: 'en-US',

  themeConfig: {
    nav: [
      { text: 'Guide', link: '/guide/introduction' },
      { text: 'Reference', link: '/reference/language' },
      { text: 'Architecture', link: '/internals/architecture' },
    ],

    sidebar: [
      {
        text: 'Guide',
        items: [
          { text: 'Introduction', link: '/guide/introduction' },
          { text: 'Installation', link: '/guide/installation' },
          { text: 'First API', link: '/guide/first-api' },
          { text: 'Visual Pages', link: '/guide/pages' },
          { text: 'UI Pages', link: '/guide/ui-pages' },
          { text: 'Core Concepts', link: '/guide/core-concepts' },
          { text: 'Project Structure', link: '/guide/project-structure' },
          { text: 'Troubleshooting', link: '/guide/troubleshooting' },
        ],
      },
      {
        text: 'Reference',
        items: [
          { text: 'Language Syntax', link: '/reference/language' },
          { text: 'Instruction Set', link: '/reference/instructions' },
          { text: 'Native Functions', link: '/reference/native-functions' },
          { text: 'HTTP Runtime', link: '/reference/http' },
          { text: 'Runtime and VM', link: '/reference/runtime' },
          { text: 'CLI and Manifest', link: '/reference/cli' },
        ],
      },
      {
        text: 'Internals',
        items: [
          { text: 'Architecture', link: '/internals/architecture' },
        ],
      },
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/MateusGX/flint' },
    ],

    footer: {
      message: 'Experimental assembly-like language for APIs and web systems.',
      copyright: 'Copyright © 2026 Mateus Martins',
    },
  },
})
