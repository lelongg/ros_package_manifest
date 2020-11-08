use once_cell::sync::Lazy;
use regex::Regex;
use roxmltree::Node;
use std::convert::TryFrom;
use thiserror::Error;

static VERSION_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[[:digit:]]+.[[:digit:]]+.[[:digit:]]+$").unwrap());

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Version {
    pub version: String,
    pub compatibility: String,
}

#[derive(Debug, Clone, Error)]
pub enum VersionError {
    #[error("no version")]
    NoVersion,
    #[error("invalid version format: {}", _0)]
    InvalidVersionFormat(String),
    #[error("invalid compatibility format: {}", _0)]
    InvalidCompatibilityFormat(String),
}

impl TryFrom<Node<'_, '_>> for Version {
    type Error = VersionError;
    fn try_from(node: Node) -> Result<Self, Self::Error> {
        let version = node.text().ok_or(VersionError::NoVersion)?;

        if !VERSION_REGEX.is_match(version) {
            return Err(VersionError::InvalidVersionFormat(version.to_string()));
        };

        let compatibility = node.attribute("compatibility").unwrap_or(version);

        if !VERSION_REGEX.is_match(compatibility) {
            return Err(VersionError::InvalidCompatibilityFormat(
                compatibility.to_string(),
            ));
        };

        Ok(Self {
            version: version.to_string(),
            compatibility: compatibility.to_string(),
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_version() {
        assert!(VERSION_REGEX.is_match("1.2.3"));
        assert!(VERSION_REGEX.is_match("11.22.33"));
        assert!(!VERSION_REGEX.is_match("12.3"));
        assert!(!VERSION_REGEX.is_match("a.2.3"));
    }
}
