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

fn html_response(html: &str) -> Result<Response> {
    let mut headers = Headers::new();
    headers.set("Content-Type", "text/html; charset=utf-8")?;
    apply_cors_headers(&mut headers)?;
    Ok(Response::ok(html)?.with_headers(headers))
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

fn render_docs_html(api_base: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta http-equiv="Content-Type" content="text/html;charset=utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Meting-API (Rust WASM)</title>
    <style>
        :root {{
            --bg: #0f172a;
            --card: #1e293b;
            --text: #f8fafc;
            --muted: #94a3b8;
            --accent: #38bdf8;
            --border: #334155;
            --code: #0b1120;
            --hover: #2dd4bf;
        }}
        @media (prefers-color-scheme: light) {{
            :root {{
                --bg: #f8fafc;
                --card: #ffffff;
                --text: #0f172a;
                --muted: #64748b;
                --accent: #0284c7;
                --border: #e2e8f0;
                --code: #f1f5f9;
                --hover: #0369a1;
            }}
        }}
        * {{ box-sizing: border-box; margin: 0; padding: 0; }}
        body {{
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
            background: var(--bg);
            color: var(--text);
            line-height: 1.6;
            padding: 2.5rem 1rem;
        }}
        .container {{
            max-width: 820px;
            margin: 0 auto;
            background: var(--card);
            border: 1px solid var(--border);
            border-radius: 14px;
            padding: 2.2rem;
            box-shadow: 0 10px 30px -5px rgba(0,0,0,0.15);
        }}
        .header {{
            display: flex;
            align-items: center;
            justify-content: space-between;
            border-bottom: 1px solid var(--border);
            padding-bottom: 1.2rem;
            margin-bottom: 1.5rem;
            flex-wrap: wrap;
            gap: 0.8rem;
        }}
        h1 {{
            font-size: 1.75rem;
            font-weight: 700;
            display: flex;
            align-items: center;
            gap: 0.6rem;
        }}
        .badge {{
            font-size: 0.72rem;
            padding: 0.2rem 0.6rem;
            background: var(--accent);
            color: #0f172a;
            font-weight: 700;
            border-radius: 9999px;
            text-transform: uppercase;
            letter-spacing: 0.5px;
        }}
        h2 {{
            font-size: 1.2rem;
            font-weight: 600;
            margin: 1.5rem 0 0.8rem;
            color: var(--accent);
        }}
        p, ul {{
            font-size: 0.95rem;
            margin-bottom: 1rem;
        }}
        table {{
            width: 100%;
            border-collapse: collapse;
            margin: 1rem 0 1.5rem;
        }}
        th, td {{
            text-align: left;
            padding: 0.65rem 0.85rem;
            border: 1px solid var(--border);
            font-size: 0.92rem;
        }}
        th {{
            background: var(--code);
            color: var(--text);
            font-weight: 600;
        }}
        code {{
            background: var(--code);
            color: var(--accent);
            padding: 0.15rem 0.4rem;
            border-radius: 4px;
            font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
            font-size: 0.88em;
        }}
        a {{
            color: var(--accent);
            text-decoration: none;
            word-break: break-all;
            transition: color 0.15s ease;
        }}
        a:hover {{
            color: var(--hover);
            text-decoration: underline;
        }}
        .example-card {{
            background: var(--code);
            border: 1px solid var(--border);
            border-radius: 8px;
            padding: 0.85rem 1rem;
            margin-bottom: 0.75rem;
            display: flex;
            flex-direction: column;
            gap: 0.35rem;
        }}
        .example-title {{
            font-size: 0.85rem;
            font-weight: 600;
            color: var(--muted);
        }}
        .footer {{
            margin-top: 2.5rem;
            padding-top: 1.2rem;
            border-top: 1px solid var(--border);
            color: var(--muted);
            font-size: 0.85rem;
            text-align: center;
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>Meting-API <span class="badge">Rust WASM</span></h1>
            <span style="font-size: 0.85rem; color: var(--muted);">Cloudflare Workers Edge API</span>
        </div>

        <h2>参数说明 (Parameter Reference)</h2>
        <table>
            <thead>
                <tr>
                    <th style="width: 20%;">参数 (Param)</th>
                    <th style="width: 30%;">取值 (Options)</th>
                    <th>说明 (Description)</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td><code>server</code></td>
                    <td>
                        <code>netease</code> (默认)<br />
                        <code>audius</code>
                    </td>
                    <td>数据源平台 (Music Streaming Service)</td>
                </tr>
                <tr>
                    <td><code>type</code></td>
                    <td>
                        <code>name</code> (歌曲名)<br />
                        <code>artist</code> (歌手名)<br />
                        <code>url</code> (音频直链 302)<br />
                        <code>pic</code> (封面图片 302)<br />
                        <code>lrc</code> (歌词 .lrc / JSON)<br />
                        <code>song</code> (单曲详情 JSON)<br />
                        <code>playlist</code> (歌单列表 JSON)<br />
                        <code>search</code> (音乐搜索 JSON)
                    </td>
                    <td>请求返回的数据类型 (Target Resource Type)</td>
                </tr>
                <tr>
                    <td><code>id</code></td>
                    <td>歌曲ID / 歌单ID / 封面ID</td>
                    <td>对应音乐平台的资源唯一识别码 (Resource ID)</td>
                </tr>
            </tbody>
        </table>

        <p>本项目基于 <a href="https://github.com/metowolf/Meting" target="_blank">metowolf/Meting</a> 与 <a href="https://github.com/injahow/meting-api" target="_blank">injahow/meting-api</a> 协议设计，使用 <strong>Rust WebAssembly</strong> 重构并部署在 <strong>Cloudflare Workers</strong> 全球边缘节点。</p>

        <h2>调用示例 (Live Examples)</h2>
        <div class="example-card">
            <span class="example-title">1. 获取歌曲播放音频链接 (type=url):</span>
            <a href="{api_base}/?server=netease&type=url&id=416892104" target="_blank">{api_base}/?server=netease&type=url&id=416892104</a>
        </div>
        <div class="example-card">
            <span class="example-title">2. 获取单曲详细信息 (type=song):</span>
            <a href="{api_base}/?server=netease&type=song&id=591321" target="_blank">{api_base}/?server=netease&type=song&id=591321</a>
        </div>
        <div class="example-card">
            <span class="example-title">3. 获取完整歌单列表 (type=playlist):</span>
            <a href="{api_base}/?server=netease&type=playlist&id=2619366284" target="_blank">{api_base}/?server=netease&type=playlist&id=2619366284</a>
        </div>
        <div class="example-card">
            <span class="example-title">4. 获取专辑封面图片 (type=pic):</span>
            <a href="{api_base}/?server=netease&type=pic&id=591321" target="_blank">{api_base}/?server=netease&type=pic&id=591321</a>
        </div>
        <div class="example-card">
            <span class="example-title">5. 获取同步歌词文件 (type=lrc):</span>
            <a href="{api_base}/?server=netease&type=lrc&id=3313423455" target="_blank">{api_base}/?server=netease&type=lrc&id=3313423455</a>
        </div>
        <div class="example-card">
            <span class="example-title">6. 获取歌曲名 (type=name):</span>
            <a href="{api_base}/?server=netease&type=name&id=416892104" target="_blank">{api_base}/?server=netease&type=name&id=416892104</a>
        </div>
        <div class="example-card">
            <span class="example-title">7. 获取歌手名 (type=artist):</span>
            <a href="{api_base}/?server=netease&type=artist&id=416892104" target="_blank">{api_base}/?server=netease&type=artist&id=416892104</a>
        </div>

        <div class="footer">
            Meting API &bull; Built with <strong>Rust + WebAssembly</strong> on Cloudflare Workers &bull; Created by Erlk1m
        </div>
    </div>
</body>
</html>"#,
        api_base = api_base
    )
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
    let base_origin = format!("{}://{}", url.scheme(), url.host_str().unwrap_or(""));
    
    // Support both root "/" and subpaths like "/meting" or "/meting/"
    let path = url.path();
    let api_base = if path.starts_with("/meting") {
        format!("{}/meting", base_origin)
    } else {
        base_origin
    };

    // Parse query parameters
    let params: HashMap<String, String> = url.query_pairs().into_owned().collect();

    let server = params.get("server").map(|s| s.as_str()).unwrap_or("netease");
    let req_type = match params.get("type") {
        Some(t) => t.as_str(),
        None => {
            // If requested format is explicitly json, return JSON summary; otherwise serve the documentation HTML page
            if params.get("format").map(|f| f.as_str()) == Some("json") {
                return json_response(&serde_json::json!({
                    "service": "Meting API (Rust WebAssembly)",
                    "author": "Erlk1m",
                    "version": "1.1.0",
                    "runtime": "Cloudflare Workers / wasm32",
                    "endpoints": {
                        "playlist": format!("{}/?server=netease&type=playlist&id=2619366284", api_base),
                        "song": format!("{}/?server=netease&type=song&id=591321", api_base),
                        "lyric": format!("{}/?server=netease&type=lrc&id=3313423455", api_base),
                        "url": format!("{}/?server=netease&type=url&id=416892104", api_base),
                        "pic": format!("{}/?server=netease&type=pic&id=591321", api_base),
                        "name": format!("{}/?server=netease&type=name&id=416892104", api_base),
                        "artist": format!("{}/?server=netease&type=artist&id=416892104", api_base),
                        "search": format!("{}/?server=netease&type=search&id=radwimps", api_base)
                    }
                }));
            }
            return html_response(&render_docs_html(&api_base));
        }
    };

    let id = params.get("id").map(|s| s.as_str()).unwrap_or("");
    if id.is_empty() && req_type != "info" {
        return error_response("Missing required parameter: id", 400);
    }

    // 2. Dispatch based on Server and Type
    match server {
        "netease" => handle_netease(req_type, id, &api_base, &params).await,
        "audius" => handle_audius(req_type, id, &api_base).await,
        other => error_response(&format!("Unsupported music server: '{}'", other), 400),
    }
}

async fn handle_netease(
    req_type: &str,
    id: &str,
    api_base: &str,
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

                    MetingSong {
                        name: t.name,
                        artist: if artist.is_empty() { "Unknown".to_string() } else { artist },
                        url: format!("{}/?server=netease&type=url&id={}", api_base, t.id),
                        pic: format!("{}/?server=netease&type=pic&id={}", api_base, t.id),
                        lrc: format!("{}/?server=netease&type=lrc&id={}", api_base, t.id),
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

                    MetingSong {
                        name: t.name,
                        artist: if artist.is_empty() { "Unknown".to_string() } else { artist },
                        url: format!("{}/?server=netease&type=url&id={}", api_base, t.id),
                        pic: format!("{}/?server=netease&type=pic&id={}", api_base, t.id),
                        lrc: format!("{}/?server=netease&type=lrc&id={}", api_base, t.id),
                    }
                })
                .collect();

            json_response(&songs)
        }

        "name" => {
            let api_url = format!("https://music.163.com/api/song/detail?ids=[{}]", id);
            let body = fetch_netease(&api_url).await?;
            let parsed: NeteaseSongDetailResponse = serde_json::from_str(&body).unwrap_or(NeteaseSongDetailResponse {
                code: 200,
                songs: vec![],
            });

            if let Some(song) = parsed.songs.first() {
                text_response(&song.name, "text/plain; charset=utf-8")
            } else {
                error_response("Song not found", 404)
            }
        }

        "artist" => {
            let api_url = format!("https://music.163.com/api/song/detail?ids=[{}]", id);
            let body = fetch_netease(&api_url).await?;
            let parsed: NeteaseSongDetailResponse = serde_json::from_str(&body).unwrap_or(NeteaseSongDetailResponse {
                code: 200,
                songs: vec![],
            });

            if let Some(song) = parsed.songs.first() {
                let artist = song
                    .artists
                    .iter()
                    .map(|a| a.name.as_str())
                    .filter(|n| !n.is_empty())
                    .collect::<Vec<_>>()
                    .join(" / ");
                text_response(&artist, "text/plain; charset=utf-8")
            } else {
                error_response("Artist not found", 404)
            }
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
                    let sized_pic = if secure_pic.contains("?") {
                        secure_pic
                    } else {
                        format!("{}?param=300y300", secure_pic)
                    };
                    return redirect_response(&sized_pic);
                }
            }

            // Fallback: direct CDN by ID
            redirect_response(&format!("https://p2.music.126.net/6y-UleORITEDbvrOLV0Q8A==/{}.jpg?param=300y300", id))
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

                    MetingSong {
                        name: t.name,
                        artist: if artist.is_empty() { "Unknown".to_string() } else { artist },
                        url: format!("{}/?server=netease&type=url&id={}", api_base, t.id),
                        pic: format!("{}/?server=netease&type=pic&id={}", api_base, t.id),
                        lrc: format!("{}/?server=netease&type=lrc&id={}", api_base, t.id),
                    }
                })
                .collect();

            json_response(&songs)
        }

        other => error_response(&format!("Unsupported operation type: '{}'", other), 400),
    }
}

async fn handle_audius(req_type: &str, id: &str, api_base: &str) -> Result<Response> {
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
                    MetingSong {
                        name: t.title,
                        artist,
                        url: format!("{}/?server=audius&type=url&id={}", api_base, t.id),
                        pic: format!("{}/?server=audius&type=pic&id={}", api_base, t.id),
                        lrc: format!("{}/?server=audius&type=lrc&id={}", api_base, t.id),
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
                let song = MetingSong {
                    name: t.title,
                    artist,
                    url: format!("{}/?server=audius&type=url&id={}", api_base, t.id),
                    pic: format!("{}/?server=audius&type=pic&id={}", api_base, t.id),
                    lrc: format!("{}/?server=audius&type=lrc&id={}", api_base, t.id),
                };
                json_response(&vec![song])
            } else {
                error_response("Track not found", 404)
            }
        }

        "name" => {
            let path = format!("/tracks/{}", id);
            let body = fetch_audius(&path).await?;
            let parsed: AudiusTrackResponse = serde_json::from_str(&body).unwrap_or(AudiusTrackResponse { data: None });
            if let Some(t) = parsed.data {
                text_response(&t.title, "text/plain; charset=utf-8")
            } else {
                error_response("Track not found", 404)
            }
        }

        "artist" => {
            let path = format!("/tracks/{}", id);
            let body = fetch_audius(&path).await?;
            let parsed: AudiusTrackResponse = serde_json::from_str(&body).unwrap_or(AudiusTrackResponse { data: None });
            if let Some(t) = parsed.data {
                let artist = t.user.map(|u| u.name).unwrap_or_else(|| "Unknown".to_string());
                text_response(&artist, "text/plain; charset=utf-8")
            } else {
                error_response("Track not found", 404)
            }
        }

        "url" => {
            let stream_url = format!("https://api.audius.co/v1/tracks/{}/stream?app_name=meting_wasm", id);
            redirect_response(&stream_url)
        }

        "pic" => {
            let path = format!("/tracks/{}", id);
            let body = fetch_audius(&path).await?;
            let parsed: AudiusTrackResponse = serde_json::from_str(&body).unwrap_or(AudiusTrackResponse { data: None });
            if let Some(t) = parsed.data {
                if let Some(artwork) = t.artwork {
                    if let Some(img) = artwork.large.or(artwork.medium).or(artwork.small) {
                        return redirect_response(&img);
                    }
                }
            }
            redirect_response("https://audius.co/assets/img/default-track-artwork.png")
        }

        "lrc" => {
            // Audius does not store synced LRC natively; return placeholder
            text_response("[00:00.00] Audius Music", "text/plain; charset=utf-8")
        }

        other => error_response(&format!("Unsupported Audius type: '{}'", other), 400),
    }
}
