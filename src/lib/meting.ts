/**
 * Meting API client — resolves music platform URLs to playable audio streams.
 *
 * Ported from Shoka player.js URL parsing + Meting API integration.
 * Supports NetEase Cloud Music, QQ Music, Audius (NCS / Trending / Playlists),
 * direct audio URLs, and R2 playlist URLs.
 */

const DEFAULT_API = 'https://163.hyc.moe/';
const CACHE_TTL = 5 * 60 * 1000; // 5 minutes

export interface MetingSong {
  name: string;
  artist: string;
  url: string;
  pic: string;
  lrc: string;
}

interface ParsedUrl {
  server: string;
  type: string;
  id: string;
}

// URL parsing rules (ported from Shoka player.js:30-47)
const URL_RULES: [RegExp, string, string][] = [
  [/music\.163\.com.*song.*id=(\d+)/, 'netease', 'song'],
  [/music\.163\.com.*album.*id=(\d+)/, 'netease', 'albumlist'],
  [/music\.163\.com.*playlist.*id=(\d+)/, 'netease', 'playlist'],
  [/music\.163\.com.*discover\/toplist.*id=(\d+)/, 'netease', 'playlist'],
  [/y\.qq\.com.*song\/(\w+)/, 'tencent', 'song'],
  [/y\.qq\.com.*album\/(\w+)/, 'tencent', 'albumlist'],
  [/y\.qq\.com.*playsquare\/(\w+)/, 'tencent', 'playlist'],
  [/y\.qq\.com.*playlist\/(\w+)/, 'tencent', 'playlist'],
];

/** Parse a music platform URL into server/type/id triple. */
export function parseMusicUrl(url: string): ParsedUrl | null {
  for (const [regex, server, type] of URL_RULES) {
    const match = url.match(regex);
    if (match?.[1]) {
      return { server, type, id: match[1] };
    }
  }
  return null;
}

/** Check if a string is a direct audio URL (mp3, ogg, wav, flac, etc.) */
function isDirectAudioUrl(url: string): boolean {
  return /\.(mp3|ogg|wav|flac|m4a|aac)(\?|$)/i.test(url);
}

/** Check if a URL is an Audius URL */
function isAudiusUrl(url: string): boolean {
  return /audius\.co/i.test(url);
}

/** Check if a URL is an R2 playlist worker URL */
function isR2PlaylistUrl(url: string): boolean {
  return /r2-playlist.*\.workers\.dev\/playlist/i.test(url);
}

/** Create a MetingSong from a direct audio URL. */
function toDirectSong(url: string): MetingSong {
  const filename = url.split('/').pop()?.split('?')[0] ?? 'Unknown';
  const name = filename
    .replace(/\.[^.]+$/, '')
    .replace(/[-_]/g, ' ')
    .replace(/%20/g, ' ');
  return { name, artist: 'Unknown', url, pic: '', lrc: '' };
}

interface CacheEntry {
  data: MetingSong[];
  timestamp: number;
}

function getCacheKey(server: string, type: string, id: string): string {
  return `meting:${server}:${type}:${id}`;
}

function getFromCache(key: string): MetingSong[] | null {
  try {
    const raw = localStorage.getItem(key);
    if (!raw) return null;
    const entry: CacheEntry = JSON.parse(raw);
    if (Date.now() - entry.timestamp > CACHE_TTL) {
      localStorage.removeItem(key);
      return null;
    }
    return entry.data;
  } catch {
    return null;
  }
}

function setCache(key: string, data: MetingSong[]): void {
  try {
    const entry: CacheEntry = { data, timestamp: Date.now() };
    localStorage.setItem(key, JSON.stringify(entry));
  } catch {
    // localStorage full or unavailable
  }
}

function isMetingSong(obj: unknown): obj is MetingSong {
  if (typeof obj !== 'object' || obj === null) return false;
  const o = obj as Record<string, unknown>;
  return typeof o.name === 'string' && typeof o.artist === 'string' && typeof o.url === 'string';
}

/** Fetch songs from R2 playlist worker. */
async function fetchR2Playlist(url: string): Promise<MetingSong[]> {
  const cacheKey = `r2playlist:${url}`;
  const cached = getFromCache(cacheKey);
  if (cached) return cached;

  const response = await fetch(url);
  if (!response.ok) throw new Error(`R2 playlist error: ${response.status}`);

  const data: unknown = await response.json();
  if (!Array.isArray(data)) return [];
  const songs = data.filter(isMetingSong) as MetingSong[];
  if (songs.length > 0) setCache(cacheKey, songs);
  return songs;
}

interface AudiusTrack {
  id: string;
  title: string;
  user?: {
    name?: string;
    handle?: string;
  };
  artwork?: {
    '150x150'?: string;
    '480x480'?: string;
    '1000x1000'?: string;
  };
}

const AUDIUS_PROVIDERS = [
  'https://api.audius.co',
  'https://discoveryprovider.audius.co',
  'https://audius-discovery-1.cultur3stake.com',
];

/** Fetch songs from Audius API (trending, user profile/tracks, or playlist). */
async function fetchAudius(url: string): Promise<MetingSong[]> {
  const cacheKey = `audius:${url}`;
  const cached = getFromCache(cacheKey);
  if (cached) return cached;

  const appName = 'webku';

  let path = '';
  try {
    const u = new URL(url);
    const pathParts = u.pathname.replace(/^\/+|\/+$/g, '').split('/').filter(Boolean);

    if (pathParts[0] === 'trending') {
      const time = u.searchParams.get('timeRange') || u.searchParams.get('time') || 'week';
      const genre = u.searchParams.get('genre');
      const genreParam = genre ? `&genre=${encodeURIComponent(genre)}` : '';
      path = `/v1/tracks/trending?app_name=${appName}&time=${time}${genreParam}&limit=50`;
    } else if (pathParts.length === 1 || (pathParts.length === 2 && pathParts[1] === 'tracks')) {
      const handle = pathParts[0];
      path = `/v1/users/handle/${encodeURIComponent(handle)}/tracks?app_name=${appName}&limit=50`;
    } else {
      path = `/v1/resolve?url=${encodeURIComponent(url)}&app_name=${appName}`;
    }
  } catch {
    return [];
  }

  for (const host of AUDIUS_PROVIDERS) {
    try {
      const apiUrl = `${host}${path}`;
      const response = await fetch(apiUrl);
      if (!response.ok) continue;

      const resJson: any = await response.json();
      let rawTracks: AudiusTrack[] = [];

      if (Array.isArray(resJson.data)) {
        rawTracks = resJson.data;
      } else if (resJson.data?.tracks && Array.isArray(resJson.data.tracks)) {
        rawTracks = resJson.data.tracks;
      } else if (resJson.data?.id && (resJson.data?.handle || resJson.data?.name) && !resJson.data?.title) {
        const userTracksUrl = `${host}/v1/users/${encodeURIComponent(resJson.data.id)}/tracks?app_name=${appName}&limit=50`;
        const userTracksRes = await fetch(userTracksUrl);
        if (userTracksRes.ok) {
          const uJson: any = await userTracksRes.json();
          if (Array.isArray(uJson.data)) {
            rawTracks = uJson.data;
          }
        }
      } else if (resJson.data?.id && resJson.data?.title) {
        rawTracks = [resJson.data];
      }

      if (rawTracks.length > 0) {
        const songs: MetingSong[] = rawTracks
          .filter((t) => t && t.id && t.title)
          .map((t) => ({
            name: t.title,
            artist: t.user?.name || 'NoCopyrightSounds',
            url: `${host}/v1/tracks/${t.id}/stream?app_name=${appName}`,
            pic: t.artwork?.['480x480'] || t.artwork?.['150x150'] || t.artwork?.['1000x1000'] || '',
            lrc: '',
          }));

        if (songs.length > 0) {
          setCache(cacheKey, songs);
          return songs;
        }
      }
    } catch {
      // Continue to next provider on network error
    }
  }

  return [];
}

/** Fetch songs from Meting API for a single parsed URL. */
export async function fetchMeting(server: string, type: string, id: string, apiUrl?: string): Promise<MetingSong[]> {
  const cacheKey = getCacheKey(server, type, id);
  const cached = getFromCache(cacheKey);
  if (cached) return cached;

  const url = new URL(apiUrl || DEFAULT_API);
  const params = new URLSearchParams({ server, type, id });
  url.search = params.toString();
  const response = await fetch(url);
  if (!response.ok) throw new Error(`Meting API error: ${response.status}`);

  const data: unknown = await response.json();
  if (!Array.isArray(data)) return [];
  const songs = data.filter(isMetingSong) as MetingSong[];
  if (songs.length > 0) setCache(cacheKey, songs);
  return songs;
}

/** Resolve multiple music URLs into a flat song list. */
export async function resolvePlaylist(urls: string[], apiUrl?: string): Promise<MetingSong[]> {
  const results = await Promise.allSettled(
    urls.map((url) => {
      if (isAudiusUrl(url)) {
        return fetchAudius(url);
      }
      if (isR2PlaylistUrl(url)) {
        return fetchR2Playlist(url);
      }
      if (isDirectAudioUrl(url)) {
        return Promise.resolve([toDirectSong(url)]);
      }
      const parsed = parseMusicUrl(url);
      if (!parsed) return Promise.resolve([]);
      return fetchMeting(parsed.server, parsed.type, parsed.id, apiUrl);
    }),
  );

  return results.flatMap((r) => (r.status === 'fulfilled' ? r.value : []));
}
