use url::{ParseError, Url};

pub async fn parse_url_path(url: &String) -> Result<String, ParseError> {
    let path = Url::parse(url)?.path().to_string();
    Ok(path)
}

pub async fn parse_url_path_segments(url: &String) -> Result<Vec<String>, ParseError> {
    let path_segments: Vec<String> = Url::parse(url)?
        .path_segments()
        .unwrap()
        .map(|s| s.to_string())
        .collect();
    Ok(path_segments)
}

pub async fn parse_url_query(url: &String) -> Result<Vec<(String, String)>, ParseError> {
    let query_pairs: Vec<(String, String)> = Url::parse(url)?
        .query_pairs()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
    Ok(query_pairs)
}

pub async fn parse_url_fragment(url: &String) -> Result<String, ParseError> {
    let fragment: String = Url::parse(url)?.fragment().map(|s| s.to_string()).unwrap();
    Ok(fragment)
}
