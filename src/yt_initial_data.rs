use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct YtInitialData {
    pub contents: Contents,
    #[serde(rename = "currentVideoEndpoint")]
    pub current_video_endpoint: CurrentVideoEndpoint,
    // TODO: frameworkUpdates
    // TODO: onResponseReceivedEndpoints
    // TODO: overlay
    // TODO: playerOverlays
    // TODO: responseContext
    // TODO: topbar
    #[serde(rename = "trackingParams")]
    pub tracking_params: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Contents {
    #[serde(rename = "twoColumnWatchNextResults")]
    TwoColumnWatchNextResults {
        // TODO: autoplay
    // TODO: results
    // TODO: secondaryResults
    },
    #[serde(rename = "twoColumnBrowseResultsRenderer")]
    TwoColumnBrowseResultsRenderer {
        // TODO
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CurrentVideoEndpoint {
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: String,
    #[serde(rename = "commandMetadata")]
    pub command_metadata: CommandMetadata,
    #[serde(rename = "watchEndpoint")]
    pub watch_endpoint: WatchEndpoint,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CommandMetadata {
    #[serde(rename = "webCommandMetadata")]
    pub web_command_metadata: WebCommandMetadata,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WebCommandMetadata {
    #[serde(rename = "rootVe")]
    pub root_ve: i32,
    pub url: String,
    #[serde(rename = "webPageType")]
    pub web_page_type: String, // TODO: could probably be an enum
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WatchEndpoint {
    #[serde(rename = "videoId")]
    pub video_id: String,
    // TODO: watchEndpointSupportedOnesieConfig
}
