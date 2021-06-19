use anyhow::Result;
use reqwest::Client as HttpClient;
use std::convert::TryInto;
use tokio::fs;
use utube::YoutubeInfo;

#[tokio::main]
async fn main() -> Result<()> {
    let http_client = HttpClient::new();
    let id = "dQw4w9WgXcQ";
    let url = format!("https://www.youtube.com/watch?v={}", id);
    let page = http_client.get(url).send().await?.text().await?;

    fs::write(format!("temp/pages/page.{}.html", id), &page).await?;

    let raw_info = utube::parse_info_raw(&page)?;

    fs::write(
        "temp/parsed/info.raw.json",
        serde_json::to_string_pretty(&raw_info)?,
    )
    .await?;

    let info: YoutubeInfo = raw_info.try_into()?;

    fs::write(
        "temp/parsed/info.json",
        serde_json::to_string_pretty(&info)?,
    )
    .await?;

    Ok(())
}
