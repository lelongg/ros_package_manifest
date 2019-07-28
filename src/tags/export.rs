use crate::PackageError;
use roxmltree::Node;
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq)]
pub struct Export {
    pub architecture_independent: bool,
    pub build_type: BuildType,
    pub deprecated: Option<String>,
    pub message_generator: Option<String>,
    pub metapackage: bool,
    pub exported: Vec<Exported>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BuildType {
    pub name: String,
    pub condition: Option<String>,
}

impl Default for BuildType {
    fn default() -> Self {
        Self {
            name: "catkin".to_string(),
            condition: None,
        }
    }
}

impl TryFrom<Node<'_, '_>> for BuildType {
    type Error = PackageError;
    fn try_from(node: Node) -> Result<Self, Self::Error> {
        Ok(Self {
            name: node
                .text()
                .map(ToString::to_string)
                .unwrap_or_else(|| BuildType::default().name),
            condition: node.attribute("condition").map(ToString::to_string),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Exported {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Deprecated {
    pub text: Option<String>,
}

impl TryFrom<Node<'_, '_>> for Export {
    type Error = PackageError;
    fn try_from(node: Node) -> Result<Self, Self::Error> {
        let mut architecture_independent = None;
        let mut build_type = None;
        let mut deprecated = None;
        let mut message_generator = None;
        let mut metapackage = None;
        let mut exported = Vec::new();

        for child in node.children().filter(Node::is_element) {
            match child.tag_name().name() {
                "architecture_independent" => architecture_independent = Some(()),
                "build_type" => build_type = Some(BuildType::try_from(child)?),
                "deprecated" => deprecated = child.text(),
                "message_generator" => message_generator = child.text(),
                "metapackage" => metapackage = Some(()),
                tag_name => exported.push(Exported {
                    name: tag_name.to_string(),
                }),
            }
        }

        Ok(Self {
            architecture_independent: architecture_independent.is_some(),
            build_type: build_type.unwrap_or_default(),
            deprecated: deprecated.map(ToString::to_string),
            message_generator: message_generator.map(ToString::to_string),
            metapackage: metapackage.is_some(),
            exported,
        })
    }
}
