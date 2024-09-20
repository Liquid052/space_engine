use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct MetaBundle<T: Bundle> {
    // pub marker: Processed,
    // pub tag_item: TagItem,
    pub sub_bundle: T,
}

impl<T> MetaBundle<T>
where
    T: Bundle,
{
    // pub fn with(mut self, key: impl Into<String>, tag: Tag) -> Self {
    //     self.tag_item.insert(key.into(), tag);
    //
    //     self
    // }
}
