use roxmltree::Node;
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct Dependency {
    pub name: String,
    pub version_lt: Option<String>,
    pub version_lte: Option<String>,
    pub version_eq: Option<String>,
    pub version_gte: Option<String>,
    pub version_gt: Option<String>,
    pub condition: Option<String>,
}

#[derive(Debug, Clone, Error)]
pub enum DependencyError {
    #[error("no name")]
    NoName,
}

impl TryFrom<Node<'_, '_>> for Dependency {
    type Error = DependencyError;
    fn try_from(node: Node) -> Result<Self, Self::Error> {
        Ok(Self {
            name: node.text().ok_or(DependencyError::NoName)?.to_string(),
            version_lt: node.attribute("version_lt").map(ToString::to_string),
            version_lte: node.attribute("version_lte").map(ToString::to_string),
            version_eq: node.attribute("version_eq").map(ToString::to_string),
            version_gte: node.attribute("version_gte").map(ToString::to_string),
            version_gt: node.attribute("version_gt").map(ToString::to_string),
            condition: node.attribute("condition").map(ToString::to_string),
        })
    }
}
