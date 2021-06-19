use anyhow::Result;
use lazy_regex::Regex;
use serde::de::DeserializeOwned;

pub fn match_regex_and_parse<T: DeserializeOwned>(
    body: &str,
    regex: &Regex,
    group: usize,
) -> Result<Option<T>> {
    if let Some(captures) = regex.captures(body) {
        Ok(Some(serde_json::from_str(
            captures.get(group).expect("invalid group").as_str(),
        )?))
    } else {
        Ok(None)
    }
}
