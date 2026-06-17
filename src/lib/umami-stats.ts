import type { UmamiConfig } from '@lib/config/types';
import type { UmamiSessionStats, UmamiStatsConfig } from '@/types/umami-stats';

const CACHE_TTL = 60 * 1000; // 1 minute

async function getSessionStats(config: UmamiStatsConfig): Promise<UmamiSessionStats> {
  const { baseUrl, path } = config;

  const url = new URL(baseUrl + '/views');
  if (path) url.searchParams.append('path', path);

  const response = await fetch(url.toString(), {
    method: 'GET',
    headers: { accept: 'application/json' },
  });

  if (!response.ok) {
    const text = await response.text().catch(() => response.statusText);
    throw new Error(`Pageview counter error: ${text}`);
  }
  return await response.json();
}

interface CacheEntry {
  value: number | null;
  expiresAt: number;
}

const cache = new Map<string, CacheEntry>();
const inflightRequests = new Map<string, Promise<number | null>>();

function getCacheKey(config: UmamiStatsConfig): string {
  return `${config.baseUrl}:${config.path ?? ''}`;
}

export function getPageviews(config: UmamiStatsConfig): Promise<number | null> {
  const key = getCacheKey(config);

  const cached = cache.get(key);
  if (cached && cached.expiresAt > Date.now()) return Promise.resolve(cached.value);

  const inflight = inflightRequests.get(key);
  if (inflight) return inflight;

  const promise = getSessionStats(config)
    .then((stats) => {
      const pv = typeof stats.pageviews === 'number' ? stats.pageviews : ((stats.pageviews as any)?.value ?? 0);
      cache.set(key, { value: pv, expiresAt: Date.now() + CACHE_TTL });
      return pv;
    })
    .catch((error) => {
      console.error('Failed to fetch pageviews:', error);
      cache.set(key, { value: null, expiresAt: Date.now() + CACHE_TTL });
      return null;
    })
    .finally(() => inflightRequests.delete(key));

  inflightRequests.set(key, promise);
  return promise;
}

function normalizePath(path: string): string {
  return path.length > 1 && path.endsWith('/') ? path.slice(0, -1) : path;
}

export function createUmamiStatsConfig(config: UmamiConfig, path?: string): UmamiStatsConfig | null {
  if (!config.enabled || !config.endpoint) return null;
  return {
    baseUrl: config.endpoint,
    websiteId: config.id || '',
    shareToken: '',
    path: path ? normalizePath(path) : undefined,
  };
}
