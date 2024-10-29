use bevy::prelude::{AssetServer, Handle, Image};
use core_pack_objects::helpers::EntityTransformer;
use core_pack_objects::prelude::{AttributeTransformer, Pack, PackTag, TagType};

#[derive(Default, Debug)]
pub struct SpritePathAttribute;

impl AttributeTransformer for SpritePathAttribute {
    fn attribute(&self) -> (&str, TagType) {
        ("sprite_path", TagType::String)
    }

    fn unpack(&self, builder: &mut EntityTransformer) {
        let mut ent = builder.main_entity_mut();
        let path: String = ent.get_mut::<Pack>().unwrap().take("sprite_path")
            .unwrap()
            .into();

        let handle = builder.world().resource::<AssetServer>()
            .load::<Image>(path);

        let ent = builder.main_entity();
        builder.commands().entity(ent)
            .insert(handle);
    }

    fn pack(&self, builder: &mut EntityTransformer) {
        let asset_server = builder.world().resource::<AssetServer>().clone();
        let mut ent = builder.main_entity_mut();

        if !ent.contains::<Handle<Image>>() {
            return;
        }
        let img = ent.get::<Handle<Image>>().unwrap();

        let path = asset_server.get_path(img).unwrap();
        let path = path.to_string();

        ent.get_mut::<Pack>().unwrap()
            .insert("sprite_path", PackTag::String(path));

        let ent = builder.main_entity();


        builder.commands().entity(ent)
            .remove::<Handle<Image>>();
    }
}
