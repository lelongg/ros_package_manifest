use crate::PackageError;
use roxmltree::Node;
use std::convert::TryFrom;
use std::str::FromStr;

pub struct Tag<T> {
    pub value: T,
}

impl<T: FromStr> FromStr for Tag<T>
where
    PackageError: From<<T as FromStr>::Err>,
{
    type Err = PackageError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Tag {
            value: T::from_str(value)?,
        })
    }
}

impl<T: ToString> ToString for Tag<T> {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

impl<T: FromStr> TryFrom<Node<'_, '_>> for Tag<T>
where
    PackageError: From<<T as FromStr>::Err>,
{
    type Error = <Tag<T> as FromStr>::Err;
    fn try_from(node: Node) -> Result<Self, Self::Error> {
        let value = node.text().ok_or(PackageError::NoValue)?;
        Self::from_str(&value)
    }
}
