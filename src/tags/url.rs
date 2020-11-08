use roxmltree::Node;
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct Url {
    pub url: String,
    pub url_type: UrlType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UrlType {
    Website,
    BugTracker,
    Repository,
}

#[derive(Debug, Clone, Error)]
pub enum UrlError {
    #[error("invalid url type: {}", _0)]
    InvalidUrlType(String),
    #[error("no url")]
    NoUrl,
}

impl TryFrom<Node<'_, '_>> for Url {
    type Error = UrlError;
    fn try_from(node: Node) -> Result<Self, Self::Error> {
        Ok(Self {
            url: node.text().ok_or(UrlError::NoUrl)?.to_string(),
            url_type: match node.attribute("type").unwrap_or("website") {
                "website" => UrlType::Website,
                "bugtracker" => UrlType::BugTracker,
                "repository" => UrlType::Repository,
                url_type => return Err(UrlError::InvalidUrlType(url_type.to_string())),
            },
        })
    }
}
