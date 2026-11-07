use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::form_urlencoded;
use worker::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct MetingSong {
    pub name: String,
    pub artist: String,
    pub url: String,
    pub pic: String,
    pub lrc: String,
}

// NetEase Data Structures
#[derive(Deserialize)]
struct NeteasePlaylistResponse {
    #[serde(default)]
    code: i32,
    playlist: Option<NeteasePlaylist>,
}

#[derive(Deserialize)]
struct NeteasePlaylist {
    #[serde(default)]
    tracks: Vec<NeteaseTrack>,
}

#[derive(Deserialize)]
struct NeteaseTrack {
    id: i64,
    #[serde(default)]
    name: String,
    #[serde(default)]
    ar: Vec<NeteaseArtist>,
    al: Option<NeteaseAlbum>,
}

#[derive(Deserialize)]
struct NeteaseArtist {
    #[serde(default)]
    name: String,
}

#[derive(Deserialize)]
struct NeteaseAlbum {
    name: Option<String>,
    #[serde(rename = "picUrl")]
    pic_url: Option<String>,
}

#[derive(Deserialize)]
struct NeteaseSongDetailResponse {
    #[serde(default)]
    code: i32,
    #[serde(default)]
    songs: Vec<NeteaseSongDetail>,
}

#[derive(Deserialize)]
struct NeteaseSongDetail {
    id: i64,
    #[serde(default)]
    name: String,
    #[serde(default)]
    artists: Vec<NeteaseArtist>,
    album: Option<NeteaseAlbumSimple>,
}

#[derive(Deserialize)]
struct NeteaseAlbumSimple {
    #[serde(rename = "picUrl")]
    pic_url: Option<String>,
}

#[derive(Deserialize)]
struct NeteaseLyricResponse {
    lrc: Option<NeteaseLrc>,
}

#[derive(Deserialize)]
struct NeteaseLrc {
    lyric: Option<String>,
}

#[derive(Deserialize)]
struct NeteaseSearchResponse {
    result: Option<NeteaseSearchResult>,
}

#[derive(Deserialize)]
struct NeteaseSearchResult {
    #[serde(default)]
    songs: Vec<NeteaseSearchSong>,
}

#[derive(Deserialize)]
struct NeteaseSearchSong {
    id: i64,
    #[serde(default)]
    name: String,
    #[serde(default)]
    ar: Vec<NeteaseArtist>,
    al: Option<NeteaseAlbum>,
}

// Audius Data Structures
#[derive(Deserialize)]
struct AudiusPlaylistResponse {
    #[serde(default)]
    data: Vec<AudiusTrack>,
}

#[derive(Deserialize)]
struct AudiusTrackResponse {
    data: Option<AudiusTrack>,
}

#[derive(Deserialize)]
struct AudiusTrack {
    id: String,
    title: String,
    user: Option<AudiusUser>,
    artwork: Option<AudiusArtwork>,
}

#[derive(Deserialize)]
struct AudiusUser {
    name: String,
}

#[derive(Deserialize)]
struct AudiusArtwork {
    #[serde(rename = "1000x1000")]
    large: Option<String>,
    #[serde(rename = "480x480")]
    medium: Option<String>,
    #[serde(rename = "150x150")]
    small: Option<String>,
}

// Helper to add standard CORS and caching headers
fn apply_cors_headers(headers: &mut Headers) -> Result<()> {
    headers.set("Access-Control-Allow-Origin", "*")?;
    headers.set("Access-Control-Allow-Methods", "GET, HEAD, OPTIONS")?;
    headers.set("Access-Control-Allow-Headers", "*")?;
    headers.set("Access-Control-Max-Age", "86400")?;
    headers.set("Cache-Control", "public, max-age=3600, s-maxage=86400")?;
    Ok(())
}

fn json_response<T: Serialize>(data: &T) -> Result<Response> {
    let json_text = serde_json::to_string(data).map_err(|e| Error::from(e.to_string()))?;
    let mut headers = Headers::new();
    headers.set("Content-Type", "application/json; charset=utf-8")?;
    apply_cors_headers(&mut headers)?;
    Ok(Response::ok(json_text)?.with_headers(headers))
}

fn text_response(text: &str, content_type: &str) -> Result<Response> {
    let mut headers = Headers::new();
    headers.set("Content-Type", content_type)?;
    apply_cors_headers(&mut headers)?;
    Ok(Response::ok(text)?.with_headers(headers))
}

fn redirect_response(target_url: &str) -> Result<Response> {
    let mut headers = Headers::new();
    headers.set("Location", target_url)?;
    apply_cors_headers(&mut headers)?;
    Ok(Response::empty()?.with_status(302).with_headers(headers))
}

fn error_response(msg: &str, status: u16) -> Result<Response> {
    let err_json = serde_json::json!({ "error": msg, "code": status });
    let mut headers = Headers::new();
    headers.set("Content-Type", "application/json; charset=utf-8")?;
    apply_cors_headers(&mut headers)?;
    Ok(Response::ok(err_json.to_string())?
        .with_status(status)
        .with_headers(headers))
}

// NetEase HTTP request helper
async fn fetch_netease(url: &str) -> Result<String> {
    let headers = Headers::new();
    headers.set(
        "User-Agent",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    )?;
    headers.set("Referer", "https://music.163.com/")?;
    headers.set("Cookie", "os=pc; osver=Microsoft-Windows-10; appver=2.9.8; channel=netease")?;

    let mut init = RequestInit::new();
    init.with_headers(headers);
    init.with_method(Method::Get);

    let req = Request::new_with_init(url, &init)?;
    let mut res = Fetch::Request(req).send().await?;
    let text = res.text().await?;
    Ok(text)
}

// Audius HTTP request helper
async fn fetch_audius(path: &str) -> Result<String> {
    let url = format!("https://api.audius.co/v1{}?app_name=meting_wasm", path);
    let headers = Headers::new();
    headers.set("User-Agent", "webku-meting/1.0")?;
    headers.set("Accept", "application/json")?;

    let mut init = RequestInit::new();
    init.with_headers(headers);
    init.with_method(Method::Get);

    let req = Request::new_with_init(&url, &init)?;
    let mut res = Fetch::Request(req).send().await?;
    res.text().await
}

#[event(fetch)]
pub async fn main(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    // 1. Handle CORS Preflight
    if req.method() == Method::Options {
        let mut headers = Headers::new();
        apply_cors_headers(&mut headers)?;
        return Ok(Response::empty()?.with_status(204).with_headers(headers));
    }

    let url = req.url()?;
    let base_url = format!("{}://{}", url.scheme(), url.host_str().unwrap_or(""));

    // Parse query parameters
    let params: HashMap<String, String> = url.query_pairs().into_owned().collect();

    let server = params.get("server").map(|s| s.as_str()).unwrap_or("netease");
    let req_type = match params.get("type") {
        Some(t) => t.as_str(),
        None => {
            // Root information page when no parameters provided
            return json_response(&serde_json::json!({
                "service": "Meting API (Rust WebAssembly)",
                "author": "Erlk1m",
                "version": "1.0.0",
                "runtime": "Cloudflare Workers / wasm32",
                "usage": {
                    "playlist": "/?server=netease&type=playlist&id=8676645748",
                    "song": "/?server=netease&type=song&id=3313423455",
                    "lyric": "/?server=netease&type=lrc&id=3313423455",
                    "url": "/?server=netease&type=url&id=3313423455",
                    "pic": "/?server=netease&type=pic&id=3313423455",
                    "search": "/?server=netease&type=search&id=radwimps"
                }
            }));
        }
    };

    let id = params.get("id").map(|s| s.as_str()).unwrap_or("");
    if id.is_empty() && req_type != "info" {
        return error_response("Missing required parameter: id", 400);
    }

    // 2. Dispatch based on Server and Type
    match server {
        "netease" => handle_netease(req_type, id, &base_url, &params).await,
        "audius" => handle_audius(req_type, id, &base_url).await,
        other => error_response(&format!("Unsupported music server: '{}'", other), 400),
    }
}

async fn handle_netease(
    req_type: &str,
    id: &str,
    base_url: &str,
    params: &HashMap<String, String>,
) -> Result<Response> {
    match req_type {
        "playlist" => {
            let api_url = format!("https://music.163.com/api/v3/playlist/detail?id={}", id);
            let body = fetch_netease(&api_url).await?;
            let parsed: NeteasePlaylistResponse = match serde_json::from_str(&body) {
                Ok(p) => p,
                Err(err) => return error_response(&format!("Failed to parse playlist: {}", err), 502),
            };

            let tracks = parsed.playlist.and_then(|p| Some(p.tracks)).unwrap_or_default();
            let songs: Vec<MetingSong> = tracks
                .into_iter()
                .map(|t| {
                    let artist = t
                        .ar
                        .into_iter()
                        .map(|a| a.name)
                        .filter(|n| !n.is_empty())
                        .collect::<Vec<_>>()
                        .join(" / ");
                    let pic = t
                        .al
                        .and_then(|a| a.pic_url)
                        .unwrap_or_default()
                        .replace("http://", "https://");

                    MetingSong {
                        name: t.name,
                        artist: if artist.is_empty() { "Unknown".to_string() } else { artist },
                        url: format!("https://music.163.com/song/media/outer/url?id={}.mp3", t.id),
                        pic,
                        lrc: format!("{}/?server=netease&type=lrc&id={}", base_url, t.id),
                    }
                })
                .collect();

            json_response(&songs)
        }

        "song" => {
            let api_url = format!("https://music.163.com/api/song/detail?ids=[{}]", id);
            let body = fetch_netease(&api_url).await?;
            let parsed: NeteaseSongDetailResponse = match serde_json::from_str(&body) {
                Ok(p) => p,
                Err(err) => return error_response(&format!("Failed to parse song: {}", err), 502),
            };

            let songs: Vec<MetingSong> = parsed
                .songs
                .into_iter()
                .map(|t| {
                    let artist = t
                        .artists
                        .into_iter()
                        .map(|a| a.name)
                        .filter(|n| !n.is_empty())
                        .collect::<Vec<_>>()
                        .join(" / ");
                    let pic = t
                        .album
                        .and_then(|a| a.pic_url)
                        .unwrap_or_default()
                        .replace("http://", "https://");

                    MetingSong {
                        name: t.name,
                        artist: if artist.is_empty() { "Unknown".to_string() } else { artist },
                        url: format!("https://music.163.com/song/media/outer/url?id={}.mp3", t.id),
                        pic,
                        lrc: format!("{}/?server=netease&type=lrc&id={}", base_url, t.id),
                    }
                })
                .collect();

            json_response(&songs)
        }

        "lrc" | "lyric" => {
            let api_url = format!(
                "https://music.163.com/api/song/lyric?id={}&lv=-1&kv=-1&tv=-1",
                id
            );
            let body = fetch_netease(&api_url).await?;
            let parsed: NeteaseLyricResponse = match serde_json::from_str(&body) {
                Ok(p) => p,
                Err(_) => NeteaseLyricResponse { lrc: None },
            };

            let lyric = parsed
                .lrc
                .and_then(|l| l.lyric)
                .unwrap_or_else(|| "[00:00.00] No lyrics available".to_string());

            // If client asked for JSON format, return object, else return raw LRC plain text
            if params.get("format").map(|f| f.as_str()) == Some("json") {
                json_response(&serde_json::json!({ "lyric": lyric }))
            } else {
                text_response(&lyric, "text/plain; charset=utf-8")
            }
        }

        "url" => {
            // Direct 302 redirect to NetEase CDN Outer MP3 stream
            let audio_url = format!("https://music.163.com/song/media/outer/url?id={}.mp3", id);
            redirect_response(&audio_url)
        }

        "pic" => {
            // Resolve cover picture from song detail and redirect
            let api_url = format!("https://music.163.com/api/song/detail?ids=[{}]", id);
            let body = fetch_netease(&api_url).await?;
            let parsed: NeteaseSongDetailResponse = serde_json::from_str(&body).unwrap_or(NeteaseSongDetailResponse {
                code: 200,
                songs: vec![],
            });

            if let Some(song) = parsed.songs.first() {
                if let Some(pic) = song.album.as_ref().and_then(|a| a.pic_url.as_ref()) {
                    let secure_pic = pic.replace("http://", "https://");
                    return redirect_response(&secure_pic);
                }
            }

            redirect_response("https://p1.music.126.net/6y-UleORITEDbvrOLV0Q8A==/5639395138885805.jpg")
        }

        "search" => {
            let encoded: String = form_urlencoded::byte_serialize(id.as_bytes()).collect();
            let api_url = format!(
                "https://music.163.com/api/cloudsearch/pc?s={}&type=1&offset=0&limit=20",
                encoded
            );
            let body = fetch_netease(&api_url).await?;
            let parsed: NeteaseSearchResponse = match serde_json::from_str(&body) {
                Ok(p) => p,
                Err(err) => return error_response(&format!("Search failed to parse: {}", err), 502),
            };

            let songs_list = parsed.result.and_then(|r| Some(r.songs)).unwrap_or_default();
            let songs: Vec<MetingSong> = songs_list
                .into_iter()
                .map(|t| {
                    let artist = t
                        .ar
                        .into_iter()
                        .map(|a| a.name)
                        .filter(|n| !n.is_empty())
                        .collect::<Vec<_>>()
                        .join(" / ");
                    let pic = t
                        .al
                        .and_then(|a| a.pic_url)
                        .unwrap_or_default()
                        .replace("http://", "https://");

                    MetingSong {
                        name: t.name,
                        artist: if artist.is_empty() { "Unknown".to_string() } else { artist },
                        url: format!("https://music.163.com/song/media/outer/url?id={}.mp3", t.id),
                        pic,
                        lrc: format!("{}/?server=netease&type=lrc&id={}", base_url, t.id),
                    }
                })
                .collect();

            json_response(&songs)
        }

        other => error_response(&format!("Unsupported operation type: '{}'", other), 400),
    }
}

async fn handle_audius(req_type: &str, id: &str, base_url: &str) -> Result<Response> {
    match req_type {
        "playlist" => {
            let path = format!("/playlists/{}/tracks", id);
            let body = fetch_audius(&path).await?;
            let parsed: AudiusPlaylistResponse = match serde_json::from_str(&body) {
                Ok(p) => p,
                Err(err) => return error_response(&format!("Failed to parse Audius playlist: {}", err), 502),
            };

            let songs: Vec<MetingSong> = parsed
                .data
                .into_iter()
                .map(|t| {
                    let artist = t.user.map(|u| u.name).unwrap_or_else(|| "Unknown".to_string());
                    let pic = t
                        .artwork
                        .and_then(|a| a.large.or(a.medium).or(a.small))
                        .unwrap_or_default();

                    MetingSong {
                        name: t.title,
                        artist,
                        url: format!("https://api.audius.co/v1/tracks/{}/stream?app_name=meting_wasm", t.id),
                        pic,
                        lrc: format!("{}/?server=audius&type=lrc&id={}", base_url, t.id),
                    }
                })
                .collect();

            json_response(&songs)
        }

        "song" => {
            let path = format!("/tracks/{}", id);
            let body = fetch_audius(&path).await?;
            let parsed: AudiusTrackResponse = match serde_json::from_str(&body) {
                Ok(p) => p,
                Err(err) => return error_response(&format!("Failed to parse Audius track: {}", err), 502),
            };

            if let Some(t) = parsed.data {
                let artist = t.user.map(|u| u.name).unwrap_or_else(|| "Unknown".to_string());
                let pic = t
                    .artwork
                    .and_then(|a| a.large.or(a.medium).or(a.small))
                    .unwrap_or_default();

                let song = MetingSong {
                    name: t.title,
                    artist,
                    url: format!("https://api.audius.co/v1/tracks/{}/stream?app_name=meting_wasm", t.id),
                    pic,
                    lrc: format!("{}/?server=audius&type=lrc&id={}", base_url, t.id),
                };
                json_response(&vec![song])
            } else {
                error_response("Track not found", 404)
            }
        }

        "url" => {
            let stream_url = format!("https://api.audius.co/v1/tracks/{}/stream?app_name=meting_wasm", id);
            redirect_response(&stream_url)
        }

        "lrc" => {
            // Audius does not store synced LRC natively; return placeholder
            text_response("[00:00.00] Audius Music", "text/plain; charset=utf-8")
        }

        other => error_response(&format!("Unsupported Audius type: '{}'", other), 400),
    }
}

