use anyhow::Result;
use lazy_regex::{lazy_regex, Lazy, Regex};

mod utils;
pub mod youtube_info;

pub use youtube_info::{YoutubeInfo, YoutubeInfoRaw};

use utils::match_regex_and_parse;

static YTCFG_REGEX: Lazy<Regex> = lazy_regex!(r"ytInitialData\s*=\s*(\{.+?\});");
static YT_INITIAL_DATA_REGEX: Lazy<Regex> = lazy_regex!(r"ytInitialData\s*=\s*(\{.+?\});");
static YT_INITIAL_PLAYER_RESPONSE_REGEX: Lazy<Regex> =
    lazy_regex!(r"ytInitialPlayerResponse\s*=\s*(\{.+?\});");

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
