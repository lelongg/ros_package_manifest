use roxmltree::Node;
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct Author {
    pub name: String,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Error)]
pub enum AuthorError {
    #[error("no name")]
    NoName,
}

impl TryFrom<Node<'_, '_>> for Author {
    type Error = AuthorError;
    fn try_from(node: Node) -> Result<Self, Self::Error> {
        Ok(Self {
            name: node.text().ok_or(AuthorError::NoName)?.to_string(),
            email: node.attribute("email").map(ToString::to_string),
        })
    }
}
