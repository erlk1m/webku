/**
 * Effective motion level for islands and vanilla scripts.
 */

export type MotionLevel = 'lively' | 'subtle' | 'reduced';

const REDUCED_MOTION_QUERY = '(prefers-reduced-motion: reduce)';
let mediaQuery: MediaQueryList | undefined;
const listeners = new Set<() => void>();

function getMediaQuery(): MediaQueryList | undefined {
  if (typeof window === 'undefined') return;
  mediaQuery ??= window.matchMedia(REDUCED_MOTION_QUERY);
  return mediaQuery;
}

export function readMotionLevel(): MotionLevel {
  if (typeof document === 'undefined') return 'reduced';
  if (getMediaQuery()?.matches) return 'reduced';
  const level = document.documentElement.dataset.motion;
  if (level === 'subtle' || level === 'reduced') return level;
  return 'lively';
}

function notify(): void {
  for (const listener of listeners) listener();
}

export function subscribeMotionLevel(onChange: () => void): () => void {
  listeners.add(onChange);
  if (listeners.size === 1) {
    getMediaQuery()?.addEventListener('change', notify);
  }
  return () => {
    listeners.delete(onChange);
    if (listeners.size === 0) {
      getMediaQuery()?.removeEventListener('change', notify);
    }
  };
}

export function isMotionDisabled(): boolean {
  return readMotionLevel() === 'reduced';
}
