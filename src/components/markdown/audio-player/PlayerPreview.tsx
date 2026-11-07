/**
 * PlayerPreview — vinyl disc with tonearm (left) + song info + centered lyrics (right).
 */

import { usePlaybackLrcIndex } from '@hooks/usePlaybackTime';
import type { MetingSong } from '@lib/meting';
import { resolveMusicCover } from '@lib/music-cover';
import type { PlaybackTimeStore } from '@lib/playback-time-store';
import { cn } from '@lib/utils';
import { memo, useEffect, useMemo, useState } from 'react';
import { parseLrc } from './LrcParser';

interface PlayerPreviewProps {
  track: MetingSong | null;
  playing: boolean;
  timeStore: PlaybackTimeStore;
  /** Line height in px — must match CSS `.audio-player-lrc p` height. @default 32 */
  lrcLineHeight?: number;
  /** Container height in px — must match CSS `.audio-player-lrc` height. @default 128 */
  lrcContainerHeight?: number;
  /** Keep the lyrics slot while fetching, so floating players don't shift their controls. */
  reserveLyrics?: boolean;
}

/** Default line height and container height (match CSS values) */
const DEFAULT_LRC_LINE_HEIGHT = 32;
const DEFAULT_LRC_CONTAINER_HEIGHT = 128;

/** Resolve LRC: if it's a URL, fetch it; otherwise use as-is. If empty, query LRCLIB for synced lyrics. */
function useLrcText(lrcSource: string | undefined, trackName?: string, artistName?: string): string {
  const [text, setText] = useState('');

  useEffect(() => {
    if (lrcSource) {
      if (lrcSource.startsWith('http')) {
        const secureUrl = lrcSource.replace(/^http:\/\//i, 'https://');
        const controller = new AbortController();
        fetch(secureUrl, { signal: controller.signal })
          .then((r) => {
            if (!r.ok) throw new Error(`HTTP ${r.status}`);
            return r.text();
          })
          .then((t) => {
            setText(t);
          })
          .catch((error: unknown) => {
            if (!(error instanceof DOMException && error.name === 'AbortError')) {
              console.warn('[LRC] Failed to fetch lyrics from', secureUrl, error);
              setText('');
            }
          });
        return () => controller.abort();
      }
      setText(lrcSource);
      return;
    }

    // Fallback: If no LRC is provided (e.g. Audius / direct audio), query LRCLIB for synced lyrics
    if (!trackName) {
      setText('');
      return;
    }

    const controller = new AbortController();
    const cleanTitle = trackName
      .replace(/\[.*?\]|\(.*?\)/g, '')
      .replace(/- Remix.*/i, '')
      .trim();

    const query = new URLSearchParams({
      track_name: cleanTitle || trackName,
    });
    if (artistName && artistName !== 'Unknown' && artistName !== 'NCS') {
      query.set('artist_name', artistName);
    }

    fetch(`https://lrclib.net/api/search?${query.toString()}`, {
      signal: controller.signal,
      headers: { 'User-Agent': 'webku-blog/1.0' },
    })
      .then((r) => (r.ok ? r.json() : []))
      .then((data) => {
        if (!controller.signal.aborted && Array.isArray(data) && data.length > 0) {
          const match = data.find((d: any) => d.syncedLyrics) || data[0];
          if (match?.syncedLyrics) {
            setText(match.syncedLyrics);
          } else if (match?.plainLyrics) {
            setText(match.plainLyrics);
          } else if (match?.instrumental) {
            setText('[00:00.00] ♪ Instrumental ♪');
          }
        }
      })
      .catch(() => {
        if (!controller.signal.aborted) setText('');
      });

    return () => controller.abort();
  }, [lrcSource, trackName, artistName]);

  return text;
}

export const PlayerPreview = memo(function PlayerPreview({
  track,
  playing,
  timeStore,
  lrcLineHeight = DEFAULT_LRC_LINE_HEIGHT,
  lrcContainerHeight = DEFAULT_LRC_CONTAINER_HEIGHT,
  reserveLyrics = false,
}: PlayerPreviewProps) {
  const source = track?.pic ?? '';
  const [cover, setCover] = useState({ source: '', url: '' });
  useEffect(() => {
    if (!source) return;
    const controller = new AbortController();
    void resolveMusicCover(source, controller.signal).then((url) => {
      if (!controller.signal.aborted) setCover({ source, url });
    });
    return () => controller.abort();
  }, [source]);
  const lrcText = useLrcText(track?.lrc, track?.name, track?.artist);
  const lrcLines = useMemo(() => parseLrc(lrcText), [lrcText]);
  const currentLrcIndex = usePlaybackLrcIndex(timeStore, lrcLines);

  const lrcCenterOffset = (lrcContainerHeight - lrcLineHeight) / 2;

  return (
    <div className="audio-player-preview">
      {/* Disc wrapper: vinyl disc + tonearm */}
      <div className="audio-player-disc-wrapper">
        <div className={cn('audio-player-disc', playing && 'spinning')}>
          {cover.source === source && cover.url ? (
            <img
              src={cover.url}
              alt={track?.name || ''}
              width={200}
              height={200}
              className="audio-player-cover"
              loading="lazy"
              decoding="async"
              draggable={false}
            />
          ) : (
            <div className="audio-player-cover audio-player-cover-placeholder" />
          )}
        </div>
        <div className={cn('audio-player-needle', playing && 'playing')}>
          <div className="audio-player-needle-arm">
            <div className="audio-player-needle-head" />
          </div>
        </div>
      </div>

      {/* Song info + lyrics */}
      <div className="audio-player-info">
        <div className="audio-player-song-name" title={track?.name}>
          {track?.name || 'No track'}
        </div>
        <div className="audio-player-artist" title={track?.artist}>
          {track?.artist || ''}
        </div>

        {/* Lyrics area — current line centered vertically */}
        {(reserveLyrics || lrcLines.length > 0) && (
          <div className="audio-player-lrc">
            <div
              className="audio-player-lrc-inner"
              style={{
                transform: `translateY(${lrcCenterOffset - Math.max(0, currentLrcIndex) * lrcLineHeight}px)`,
              }}
            >
              {lrcLines.map((line, i) => (
                <p key={`${line.time}-${i}`} className={cn(i === currentLrcIndex && 'current')}>
                  {line.text || '\u00A0'}
                </p>
              ))}
            </div>
          </div>
        )}
      </div>
    </div>
  );
});
