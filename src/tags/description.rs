use crate::PackageError;
use roxmltree::Node;
use std::convert::TryFrom;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Description {
    pub description: String,
}

impl ToString for Description {
    fn to_string(&self) -> String {
        self.description.clone()
    }
}

impl TryFrom<Node<'_, '_>> for Description {
    type Error = PackageError;
    fn try_from(node: Node) -> Result<Self, Self::Error> {
        let description = node
            .descendants()
            .filter(Node::is_text)
            .filter_map(|node| node.text())
            .collect::<String>()
            .trim()
            .to_string();

        Ok(Self { description })
    }
}
