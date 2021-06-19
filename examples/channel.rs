use anyhow::Result;
use reqwest::Client as HttpClient;
use std::convert::TryInto;
use tokio::fs;
use utube::YoutubeInfo;

#[tokio::main]
async fn main() -> Result<()> {
    let http_client = HttpClient::new();

    let channel = "UC7_YxT-KID8kRbqZo7MyscQ"; // Markiplier

    let url = format!("https://www.youtube.com/channel/{}", channel);
    let page = http_client.get(url).send().await?.text().await?;

    let raw_info = utube::parse_info_raw(&page)?;

    fs::write(
        "temp/parsed/info.raw.channel.json",
        serde_json::to_string_pretty(&raw_info)?,
    )
    .await?;

    let info: YoutubeInfo = raw_info.try_into()?;

    fs::write(
        "temp/parsed/info.channel.json",
        serde_json::to_string_pretty(&info)?,
    )
    .await?;
    Ok(())
}
