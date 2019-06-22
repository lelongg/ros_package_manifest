use roxmltree::Node;
use std::convert::TryFrom;

pub trait GetTag {
    fn get_tag<T>(
        &self,
        tag_name: impl AsRef<str>,
    ) -> Option<Result<T, <T as TryFrom<Self>>::Error>>
    where
        T: TryFrom<Self>,
        Self: Sized;

    fn get_tags<T>(&self, tag_name: impl AsRef<str>) -> Result<Vec<T>, <T as TryFrom<Self>>::Error>
    where
        T: TryFrom<Self>,
        Self: Sized;
}

impl<'a, 'b> GetTag for Node<'a, 'b> {
    fn get_tag<T>(
        &self,
        tag_name: impl AsRef<str>,
    ) -> Option<Result<T, <T as TryFrom<Self>>::Error>>
    where
        T: TryFrom<Self>,
        Self: Sized,
    {
        self.descendants()
            .filter(|node| node.is_element() && node.tag_name().name() == tag_name.as_ref())
            .map(T::try_from)
            .nth(0)
    }

    fn get_tags<T>(&self, tag_name: impl AsRef<str>) -> Result<Vec<T>, <T as TryFrom<Self>>::Error>
    where
        T: TryFrom<Self>,
        Self: Sized,
    {
        self.descendants()
            .filter(|node| node.is_element() && node.tag_name().name() == tag_name.as_ref())
            .map(T::try_from)
            .collect()
    }
}
