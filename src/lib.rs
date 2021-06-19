use anyhow::Result;
use lazy_regex::{lazy_regex, Lazy, Regex};
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};

mod yt_initial_data;
mod yt_initial_player_response;
mod ytcfg;

pub use yt_initial_data::YtInitialData;
pub use yt_initial_player_response::YtInitialPlayerResponse;
pub use ytcfg::Ytcfg;

static YTCFG_REGEX: Lazy<Regex> = lazy_regex!(r"ytInitialData\s*=\s*(\{.+?\});");
static YT_INITIAL_DATA_REGEX: Lazy<Regex> = lazy_regex!(r"ytInitialData\s*=\s*(\{.+?\});");
static YT_INITIAL_PLAYER_RESPONSE_REGEX: Lazy<Regex> =
    lazy_regex!(r"ytInitialPlayerResponse\s*=\s*(\{.+?\});");

#[derive(Debug, Deserialize, Serialize)]
pub struct YoutubeInfo {
    pub yt_initial_data: Option<YtInitialData>,
    pub yt_initial_player_response: Option<YtInitialPlayerResponse>,
    pub ytcfg: Option<Ytcfg>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct YoutubeInfoRaw {
    pub yt_initial_data: Option<String>,
    pub yt_initial_player_response: Option<String>,
    pub ytcfg: Option<String>,
}

impl ToString for YoutubeInfoRaw {
    fn to_string(&self) -> String {
        format!(
            "[{},{},{}]",
            self.yt_initial_data.as_ref().unwrap_or(&"{}".to_string()),
            self.yt_initial_player_response
                .as_ref()
                .unwrap_or(&"{}".to_string()),
            self.ytcfg.as_ref().unwrap_or(&"{}".to_string()),
        )
    }
}

impl TryFrom<YoutubeInfoRaw> for YoutubeInfo {
    type Error = anyhow::Error;

    fn try_from(value: YoutubeInfoRaw) -> Result<Self, Self::Error> {
        let yt_initial_data: Option<YtInitialData> = value
            .yt_initial_data
            .map(|s| serde_json::from_str(&s))
            .transpose()?;

        let yt_initial_player_response: Option<YtInitialPlayerResponse> = value
            .yt_initial_player_response
            .map(|s| serde_json::from_str(&s))
            .transpose()?;

        let ytcfg: Option<Ytcfg> = value.ytcfg.map(|s| serde_json::from_str(&s)).transpose()?;

        Ok(YoutubeInfo {
            yt_initial_data,
            yt_initial_player_response,
            ytcfg,
        })
    }
}

pub fn parse_raw_info(page: &str) -> YoutubeInfoRaw {
    let yt_initial_data = match_regex(&page, &YT_INITIAL_DATA_REGEX, 1);
    let yt_initial_player_response = match_regex(&page, &YT_INITIAL_PLAYER_RESPONSE_REGEX, 1);
    let ytcfg = match_regex(&page, &YTCFG_REGEX, 1);

    YoutubeInfoRaw {
        yt_initial_data,
        yt_initial_player_response,
        ytcfg,
    }
}

pub fn parse_info(page: &str) -> Result<YoutubeInfo> {
    let youtube_info: YoutubeInfo = parse_raw_info(page).try_into()?;
    Ok(youtube_info)
}

fn match_regex(body: &str, regex: &Regex, group: usize) -> Option<String> {
    if let Some(captures) = regex.captures(body) {
        let data = captures
            .get(group)
            .expect("invalid group")
            .as_str()
            .to_string();
        Some(data)
    } else {
        None
    }
}
