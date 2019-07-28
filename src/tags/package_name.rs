use crate::PackageError;
use once_cell::sync::Lazy;
use regex::Regex;
use std::str::FromStr;

static PACKAGE_NAME_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[[:lower:]][[:word:]]*$").unwrap());

#[derive(Clone, Debug, PartialEq)]
pub struct PackageName {
    pub name: String,
}

impl ToString for PackageName {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

impl FromStr for PackageName {
    type Err = PackageError;
    fn from_str(name: &str) -> Result<Self, Self::Err> {
        if PACKAGE_NAME_REGEX.is_match(name) {
            Ok(PackageName {
                name: name.to_string(),
            })
        } else {
            Err(PackageError::InvalidName(name.to_string()))
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_package_name() {
        assert!(PACKAGE_NAME_REGEX.is_match("roscpp"));
        assert!(PACKAGE_NAME_REGEX.is_match("roscpp1"));
        assert!(PACKAGE_NAME_REGEX.is_match("roscpp1A"));
        assert!(PACKAGE_NAME_REGEX.is_match("ros_cpp"));
        assert!(!PACKAGE_NAME_REGEX.is_match("1roscpp"));
        assert!(!PACKAGE_NAME_REGEX.is_match("Aroscpp"));
        assert!(!PACKAGE_NAME_REGEX.is_match("_roscpp"));
    }
}
