use clap::Parser;
use futures::stream::{self, StreamExt};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(long, value_name = "FILE")]
    urls_file: PathBuf,

    #[arg(long, default_value_t = 16)]
    concurrency: usize,

    #[arg(long)]
    referer: Option<String>,

    #[arg(long)]
    output_dir: Option<PathBuf>,
}

fn read_urls(path: &PathBuf) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut urls = Vec::new();
    for line in reader.lines() {
        let l = line?;
        let trimmed = l.trim();
        if !trimmed.is_empty() && !trimmed.starts_with('#') {
            urls.push(trimmed.to_string());
        }
    }
    Ok(urls)
}

fn build_headers(args: &Args) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("Facecloud-UrlRunner/0.1"),
    );
    if let Some(ref r) = args.referer {
        if let Ok(v) = HeaderValue::from_str(r) {
            headers.insert("Referer", v);
        }
    }
    headers
}

fn sanitize_filename(url: &str) -> String {
    let parsed = url::Url::parse(url);
    if let Ok(u) = parsed {
        if let Some(name) = u.path_segments().and_then(|s| s.last()).filter(|s| !s.is_empty()) {
            return name.replace(|c: char| c == '?' || c == '&' || c == '#', "_");
        }
        return u.host_str()
            .unwrap_or("index")
            .replace(|c: char| !c.is_ascii_alphanumeric(), "_");
    }
    "output".to_string()
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let urls = read_urls(&args.urls_file)?;

    let client = reqwest::Client::builder()
        .default_headers(build_headers(&args))
        .build()?;

    let out_dir = args
        .output_dir
        .clone()
        .unwrap_or_else(|| PathBuf::from("facecloud_html"));

    std::fs::create_dir_all(&out_dir)?;

    stream::iter(urls.into_iter().map(|url| {
        let client = client.clone();
        let out_dir = out_dir.clone();
        async move {
            let resp = client.get(&url).send().await?;
            let bytes = resp.bytes().await?;
            let mut path = out_dir.clone();
            let name = sanitize_filename(&url);
            path.push(format!("{}.html", name));
            std::fs::write(&path, &bytes)?;
            Ok::<(), anyhow::Error>(())
        }
    }))
    .buffer_unordered(args.concurrency)
    .collect::<Vec<_>>()
    .await;

    Ok(())
}
