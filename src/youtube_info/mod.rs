use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

pub mod yt_initial_data;
pub mod yt_initial_player_response;
pub mod ytcfg;

pub use self::{
    yt_initial_data::YtInitialData, yt_initial_player_response::YtInitialPlayerResponse,
    ytcfg::Ytcfg,
};

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
