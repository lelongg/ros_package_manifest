use roxmltree::Node;
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct Maintainer {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, Error)]
pub enum MaintainerError {
    #[error("no name")]
    NoName,
    #[error("no email")]
    NoEmail,
}

impl TryFrom<Node<'_, '_>> for Maintainer {
    type Error = MaintainerError;
    fn try_from(node: Node) -> Result<Self, Self::Error> {
        Ok(Self {
            name: node.text().ok_or(MaintainerError::NoName)?.to_string(),
            email: node
                .attribute("email")
                .ok_or(MaintainerError::NoEmail)?
                .to_string(),
        })
    }
}
