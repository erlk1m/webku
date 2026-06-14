import { defineConfig } from 'emdash'

export default defineConfig({
  // Lokasi konten blog
  contentDir: 'src/content/blog',

  // Frontmatter schema (sesuaikan dengan skema astro-koharu)
  schema: {
    title: { type: 'string', required: true },
    date: { type: 'date', required: true },
    categories: { type: 'array', items: { type: 'string' } },
    tags: { type: 'array', items: { type: 'string' } },
    draft: { type: 'boolean', default: false },
    pinned: { type: 'boolean', default: false },
    cover: { type: 'image' },
    description: { type: 'string' },
  },

  // Media uploads
  publicDir: 'public',

  // Git integration
  git: {
    autoCommit: true,
    commitMessage: 'content: update {{title}}',
  },
})
