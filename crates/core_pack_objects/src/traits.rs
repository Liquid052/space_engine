use std::fmt::Debug;
use crate::compos::TagType;
use crate::helpers::EntityTransformer;

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

pub trait GeneralTransformer: Send + Debug + Sync + 'static {
    fn attribute(&self) -> (&str, TagType);

    fn unpack(&self, builder: &mut EntityTransformer);


    fn pack(&self, builder: &mut EntityTransformer);
}