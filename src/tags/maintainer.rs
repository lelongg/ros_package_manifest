use roxmltree::Node;
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct Maintainer {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone)]
pub enum MaintainerError {
    NoName,
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
