use neo_mime::MediaType;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Deserialize, Serialize)]
pub struct YtInitialPlayerResponse {
    /// We do not care about this field as it contains tracking informations. Skipped.
    #[serde(rename = "responseContext", skip)]
    pub response_context: serde_json::Value,
    /// Unsure if this fields is necessary or not. Also skipped for now.
    #[serde(rename = "playabilityStatus", skip)]
    pub playability_status: serde_json::Value,
    #[serde(rename = "streamingData")]
    pub streaming_data: StreamingData,
    /// We do not care about this field as it contains ad informations. Skipped.
    #[serde(rename = "playerAds", skip)]
    pub player_ads: serde_json::Value,
    /// We do not care about this field as it contains tracking informations. Skipped.
    #[serde(rename = "playbackTracking", skip)]
    pub playback_tracking: serde_json::Value,
    /// Contains subtitles informations
    pub captions: Option<serde_json::Value>, // TODO: write appropriate struct
    #[serde(rename = "videoDetails")]
    pub video_details: VideoDetails,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StreamingData {
    #[serde(rename = "expiresInSeconds")]
    pub expires_in_seconds: String,
    pub formats: Vec<Formats>,
    #[serde(rename = "adaptiveFormats")]
    pub adaptive_formats: Vec<AdaptiveFormats>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Formats {
    Format {
        itag: u32,
        url: Url,
        #[serde(rename = "mimeType")]
        mime_type: MediaType,
        bitrate: u32,
        width: u32,
        height: u32,
        #[serde(rename = "lastModified")]
        last_modified: String,
        #[serde(rename = "contentLength")]
        content_length: Option<String>,
        quality: String,
        fps: u32,
        #[serde(rename = "qualityLabel")]
        quality_label: String,
        #[serde(rename = "projectionType")]
        projection_type: String,
        #[serde(rename = "averageBitrate")]
        average_bitrate: Option<u32>,
        #[serde(rename = "audioQuality")]
        audio_quality: AudioQuality,
        #[serde(rename = "approxDurationMs")]
        approx_duration_ms: String,
        #[serde(rename = "audioSampleRate")]
        audio_sample_rate: String,
        #[serde(rename = "audioChannels")]
        audio_channels: u32,
    },
    CipheredFormat {
        itag: u32,
        #[serde(rename = "mimeType")]
        mime_type: MediaType,
        bitrate: u32,
        width: u32,
        height: u32,
        #[serde(rename = "lastModified")]
        last_modified: String,
        #[serde(rename = "contentLength")]
        content_length: Option<String>,
        quality: String,
        fps: u32,
        #[serde(rename = "qualityLabel")]
        quality_label: String,
        #[serde(rename = "projectionType")]
        projection_type: String,
        #[serde(rename = "averageBitrate")]
        average_bitrate: Option<u32>,
        #[serde(rename = "audioQuality")]
        audio_quality: AudioQuality,
        #[serde(rename = "approxDurationMs")]
        approx_duration_ms: String,
        #[serde(rename = "audioSampleRate")]
        audio_sample_rate: String,
        #[serde(rename = "audioChannels")]
        audio_channels: u32,
        #[serde(rename = "signatureCipher")]
        signature_cipher: String,
    },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AdaptiveFormats {
    Video {
        itag: u32,
        url: Url,
        #[serde(rename = "mimeType")]
        mime_type: MediaType,
        bitrate: u32,
        width: u32,
        height: u32,
        #[serde(rename = "initRange")]
        init_range: Range,
        #[serde(rename = "indexRange")]
        index_range: Range,
        #[serde(rename = "lastModified")]
        last_modified: String,
        #[serde(rename = "contentLength")]
        content_length: String,
        quality: String,
        fps: u32,
        #[serde(rename = "qualityLabel")]
        quality_label: String,
        #[serde(rename = "projectionType")]
        projection_type: String,
        #[serde(rename = "averageBitrate")]
        average_bitrate: Option<u32>,
        #[serde(rename = "colorInfo")]
        color_info: Option<ColorInfo>,
        #[serde(rename = "approxDurationMs")]
        approx_duration_ms: String,
    },
    CipheredVideo {
        itag: u32,
        #[serde(rename = "mimeType")]
        mime_type: MediaType,
        bitrate: u32,
        width: u32,
        height: u32,
        #[serde(rename = "initRange")]
        init_range: Range,
        #[serde(rename = "indexRange")]
        index_range: Range,
        #[serde(rename = "lastModified")]
        last_modified: String,
        #[serde(rename = "contentLength")]
        content_length: String,
        quality: String,
        fps: u32,
        #[serde(rename = "qualityLabel")]
        quality_label: String,
        #[serde(rename = "projectionType")]
        projection_type: String,
        #[serde(rename = "averageBitrate")]
        average_bitrate: Option<u32>,
        #[serde(rename = "colorInfo")]
        color_info: Option<ColorInfo>,
        #[serde(rename = "approxDurationMs")]
        approx_duration_ms: String,
        #[serde(rename = "signatureCipher")]
        signature_cipher: String,
    },
    Audio {
        itag: u32,
        url: Url,
        #[serde(rename = "mimeType")]
        mime_type: MediaType,
        bitrate: u32,
        #[serde(rename = "initRange")]
        init_range: Range,
        #[serde(rename = "indexRange")]
        index_range: Range,
        #[serde(rename = "lastModified")]
        last_modified: String,
        #[serde(rename = "contentLength")]
        content_length: String,
        quality: String,
        #[serde(rename = "projectionType")]
        projection_type: String,
        #[serde(rename = "averageBitrate")]
        average_bitrate: Option<u32>,
        #[serde(rename = "highReplication")]
        high_replication: Option<bool>,
        #[serde(rename = "audioQuality")]
        audio_quality: AudioQuality,
        #[serde(rename = "approxDurationMs")]
        approx_duration_ms: String,
        #[serde(rename = "audioSampleRate")]
        audio_sample_rate: String,
        #[serde(rename = "audioChannels")]
        audio_channels: u32,
        #[serde(rename = "loudnessDb")]
        loudness_db: Option<f32>,
    },

    CipheredAudio {
        itag: u32,
        #[serde(rename = "mimeType")]
        mime_type: MediaType,
        bitrate: u32,
        #[serde(rename = "initRange")]
        init_range: Range,
        #[serde(rename = "indexRange")]
        index_range: Range,
        #[serde(rename = "lastModified")]
        last_modified: String,
        #[serde(rename = "contentLength")]
        content_length: String,
        quality: String,
        #[serde(rename = "projectionType")]
        projection_type: String,
        #[serde(rename = "averageBitrate")]
        average_bitrate: Option<u32>,
        #[serde(rename = "highReplication")]
        high_replication: Option<bool>,
        #[serde(rename = "audioQuality")]
        audio_quality: AudioQuality,
        #[serde(rename = "approxDurationMs")]
        approx_duration_ms: String,
        #[serde(rename = "audioSampleRate")]
        audio_sample_rate: String,
        #[serde(rename = "audioChannels")]
        audio_channels: u32,
        #[serde(rename = "loudnessDb")]
        loudness_db: Option<f32>,
        #[serde(rename = "signatureCipher")]
        signature_cipher: String,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Range {
    pub start: String,
    pub end: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ColorInfo {
    pub primaries: Option<String>,
    #[serde(rename = "transferCharacteristics")]
    pub transfer_characteristics: Option<String>,
    #[serde(rename = "matrixCoefficients")]
    pub matrix_coefficients: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AudioQuality {
    #[serde(rename = "AUDIO_QUALITY_LOW")]
    AudioQualityLow,
    #[serde(rename = "AUDIO_QUALITY_MEDIUM")]
    AudioQualityMedium,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VideoDetails {
    #[serde(rename = "videoId")]
    pub video_id: String,
    pub title: String,
    #[serde(rename = "lengthSeconds")]
    pub length_seconds: String,
    pub keywords: Option<Vec<String>>,
    #[serde(rename = "channelId")]
    pub channel_id: String,
    #[serde(rename = "isOwnerViewing")]
    pub is_owner_viewing: bool,
    #[serde(rename = "shortDescription")]
    pub short_description: String,
    #[serde(rename = "isCrawlable")]
    pub is_crawlable: bool,
    // pub thumbnail
    #[serde(rename = "averageRating")]
    pub average_rating: f64,
    #[serde(rename = "allowRatings")]
    pub allow_ratings: bool,
    #[serde(rename = "viewCount")]
    pub view_count: String,
    pub author: String,
    #[serde(rename = "isPrivate")]
    pub is_private: bool,
    #[serde(rename = "isUnpluggedCorpus")]
    pub is_unplugged_corpus: bool,
    #[serde(rename = "isLiveContent")]
    pub is_live_content: bool,
}
