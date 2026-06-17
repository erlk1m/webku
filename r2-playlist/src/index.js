export default {
  async fetch(request, env) {
    const url = new URL(request.url);
    const origin = request.headers.get('Origin') || '*';

    const corsHeaders = {
      'Access-Control-Allow-Origin': origin,
      'Access-Control-Allow-Methods': 'GET, OPTIONS',
      'Access-Control-Allow-Headers': '*',
    };

    if (request.method === 'OPTIONS') {
      return new Response(null, { headers: corsHeaders });
    }

    // GET /playlist?prefix=dj/
    // Returns JSON array of { name, url, size }
    const prefix = url.searchParams.get('prefix') || '';

    try {
      const listed = await env.MEDIA.list({ prefix, delimiter: '' });
      const files = [];

      for (const obj of listed.objects) {
        // Only include audio files
        if (/\.(mp3|ogg|wav|flac|m4a|aac)$/i.test(obj.key)) {
          const name = obj.key
            .split('/')
            .pop()
            .replace(/\.[^.]+$/, '')
            .replace(/[-_]/g, ' ')
            .replace(/%20/g, ' ')
            .replace(/\s+/g, ' ')
            .trim();

          // Build public URL - Ganti dengan domain R2 publik kamu
          const publicUrl = `https://pub-a9aa841c2e964eeb92906c54952521c7.r2.dev/${encodeURIComponent(obj.key).replace(/%2F/g, '/')}`;

          files.push({
            name,
            artist: 'Unknown',
            url: publicUrl,
            pic: '',
            lrc: '',
            size: obj.size,
          });
        }
      }

      return new Response(JSON.stringify(files), {
        headers: {
          'Content-Type': 'application/json',
          ...corsHeaders,
          'Cache-Control': 'public, max-age=3600',
        },
      });
    } catch (e) {
      return new Response(JSON.stringify({ error: e.message }), {
        status: 500,
        headers: { 'Content-Type': 'application/json', ...corsHeaders },
      });
    }
  },
};
