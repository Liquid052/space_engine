use crate::components::TagType;
use crate::helpers::EntityTransformer;
use std::fmt::Debug;

pub trait PackTransformer: Send + Debug + Sync + 'static {
    fn type_name(&self) -> &str;

    // list of attributes/tags that are required by the builder. Also for type checks of Tag variants
    fn required(&self) -> &[(&str, TagType)] {
        &[]
    }
    fn optional(&self) -> &[(&str, TagType)] {
        &[]
    }

    fn unpack(&self, builder: &mut EntityTransformer);


    fn pack(&self, builder: &mut EntityTransformer);
}

pub trait AttributeTransformer: Send + Debug + Sync + 'static {
    fn attribute(&self) -> (&str, TagType);

    fn unpack(&self, builder: &mut EntityTransformer);


    fn pack(&self, builder: &mut EntityTransformer);
}