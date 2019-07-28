use roxmltree::Node;
use std::convert::TryFrom;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct License {
    pub license: String,
    pub file: Option<String>,
}

#[derive(Debug, Clone)]
pub enum LicenseError {
    NoLicense,
}

impl TryFrom<Node<'_, '_>> for License {
    type Error = LicenseError;
    fn try_from(node: Node) -> Result<Self, Self::Error> {
        Ok(Self {
            license: node.text().ok_or(LicenseError::NoLicense)?.to_string(),
            file: node.attribute("file").map(ToString::to_string),
        })
    }
}
