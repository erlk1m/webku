import { collection, config, fields } from '@keystatic/core';

export default config({
  storage: {
    kind: 'local',
  },
  collections: {
    blog: collection({
      label: 'Blog',
      slugField: 'title',
      path: 'src/content/blog/**',
      format: { contentField: 'content' },
      schema: {
        title: fields.slug({ name: { label: 'Judul' } }),
        description: fields.text({ label: 'Deskripsi' }),
        link: fields.text({ label: 'Link URL' }),
        date: fields.datetime({ label: 'Tanggal', validation: { isRequired: true } }),
        updated: fields.datetime({ label: 'Tanggal Update' }),
        cover: fields.text({ label: 'Cover Image URL' }),
        tags: fields.multiselect({
          label: 'Tags',
          options: [
            { label: 'armbian', value: 'armbian' },
            { label: 'AI', value: 'AI' },
            { label: 'xiaomi', value: 'xiaomi' },
            { label: 'mimo', value: 'mimo' },
            { label: 'llm', value: 'llm' },
            { label: 'docker', value: 'docker' },
            { label: 'rust', value: 'rust' },
            { label: 'linux', value: 'linux' },
            { label: 'tutorial', value: 'tutorial' },
          ],
        }),
        subtitle: fields.text({ label: 'Subtitle' }),
        catalog: fields.checkbox({ label: 'Catalog', defaultValue: true }),
        categories: fields.multiselect({
          label: 'Kategori',
          options: [
            { label: 'AI', value: 'AI' },
            { label: 'armbian', value: 'armbian' },
            { label: 'tutorial', value: 'tutorial' },
            { label: 'rust', value: 'rust' },
            { label: 'pengetahuan', value: 'pengetahuan' },
          ],
        }),
        sticky: fields.checkbox({ label: 'Sticky' }),
        draft: fields.checkbox({ label: 'Draft' }),
        tocNumbering: fields.checkbox({ label: 'TOC Numbering', defaultValue: true }),
        excludeFromSummary: fields.checkbox({ label: 'Exclude from Summary' }),
        math: fields.checkbox({ label: 'Math' }),
        quiz: fields.checkbox({ label: 'Quiz' }),
        password: fields.text({ label: 'Password' }),
        keywords: fields.array(fields.text({ label: 'Keyword' }), { label: 'Keywords', itemLabel: (props) => props.value }),
        content: fields.markdoc({
          label: 'Konten',
          extension: 'mdoc',
        }),
      },
    }),
  },
});
