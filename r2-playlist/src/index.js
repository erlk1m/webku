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

    // Serve audio file from R2 directly
    // GET /file?key=dj/song.mp3
    if (url.pathname === '/file') {
      const key = url.searchParams.get('key');
      if (!key) {
        return new Response('Missing key param', { status: 400 });
      }

      const obj = await env.MEDIA.get(key);
      if (!obj) {
        return new Response('File not found', { status: 404 });
      }

      const headers = new Headers(corsHeaders);
      headers.set('Content-Type', 'audio/mpeg');
      headers.set('Cache-Control', 'public, max-age=86400');
      headers.set('Accept-Ranges', 'bytes');

      return new Response(obj.body, { headers });
    }

    // GET /playlist - list all audio files
    const prefix = url.searchParams.get('prefix') || '';

    try {
      const listed = await env.MEDIA.list({ prefix, delimiter: '' });
      const files = [];

      for (const obj of listed.objects) {
        if (/\.(mp3|ogg|wav|flac|m4a|aac)$/i.test(obj.key)) {
          const name = obj.key
            .split('/')
            .pop()
            .replace(/\.[^.]+$/, '')
            .replace(/%20/g, ' ')
            .replace(/[-_]/g, ' ')
            .replace(/\s+/g, ' ')
            .trim();

          // URL ke worker sendiri untuk serve file
          const workerUrl = `https://media.erlkim.web.id/${encodeURIComponent(obj.key).replace(/%2F/g, '/')}`;

          files.push({
            name,
            artist: 'Unknown',
            url: workerUrl,
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
