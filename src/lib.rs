use anyhow::Result;
use lazy_regex::{lazy_regex, Lazy, Regex};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

mod utils;
pub mod yt_initial_data;
pub mod yt_initial_player_response;
pub mod ytcfg;

use utils::match_regex_and_parse;

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
    pub yt_initial_data: Option<serde_json::Value>,
    pub yt_initial_player_response: Option<serde_json::Value>,
    pub ytcfg: Option<serde_json::Value>,
}

impl TryFrom<YoutubeInfoRaw> for YoutubeInfo {
    type Error = anyhow::Error;

    fn try_from(value: YoutubeInfoRaw) -> Result<Self, Self::Error> {
        let yt_initial_data: Option<YtInitialData> = value
            .yt_initial_data
            .map(|s| serde_json::from_value(s))
            .transpose()?;

        let yt_initial_player_response: Option<YtInitialPlayerResponse> = value
            .yt_initial_player_response
            .map(|s| serde_json::from_value(s))
            .transpose()?;

        let ytcfg: Option<Ytcfg> = value.ytcfg.map(|s| serde_json::from_value(s)).transpose()?;

        Ok(YoutubeInfo {
            yt_initial_data,
            yt_initial_player_response,
            ytcfg,
        })
    }
}

pub fn parse_info_raw(page: &str) -> Result<YoutubeInfoRaw> {
    Ok(YoutubeInfoRaw {
        yt_initial_data: match_regex_and_parse(&page, &YT_INITIAL_DATA_REGEX, 1)?,
        yt_initial_player_response: match_regex_and_parse(
            &page,
            &YT_INITIAL_PLAYER_RESPONSE_REGEX,
            1,
        )?,
        ytcfg: match_regex_and_parse(&page, &YTCFG_REGEX, 1)?,
    })
}

pub fn parse_info(page: &str) -> Result<YoutubeInfo> {
    Ok(YoutubeInfo {
        yt_initial_data: match_regex_and_parse(&page, &YT_INITIAL_DATA_REGEX, 1)?,
        yt_initial_player_response: match_regex_and_parse(
            &page,
            &YT_INITIAL_PLAYER_RESPONSE_REGEX,
            1,
        )?,
        ytcfg: match_regex_and_parse(&page, &YTCFG_REGEX, 1)?,
    })
}
