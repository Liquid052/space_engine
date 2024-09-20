// example demonstrating use of pack/unpack feature
extern crate engine;
extern crate bevy_inspector_egui;
extern crate bevy;

use bevy_inspector_egui::quick::*;
use bevy_inspector_egui::egui;
use engine::prelude::*;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiContexts;
use bevy_inspector_egui::egui::Color32;

#[derive(Component, Reflect, Copy, Clone)]
#[reflect(Component)]
pub struct Block {
    // render
    pub pages:      u8,
    // logic
    pub durability: u8,
    pub hp:         u16,
}

#[derive(Default, Debug)]
pub struct BlockPack;

impl PackTransformer for BlockPack {
    fn type_name(&self) -> &str {
        "block"
    }

    fn required(&self) -> &[(&str, TagType)] {
        &[
            ("sprite_path", TagType::String),
            ("pages",       TagType::U8),
            ("durability",  TagType::U8),
            ("hp",          TagType::U16),
        ]
    }
    fn optional(&self) -> &[(&str, TagType)] {
        &[("name",     TagType::String)]
    }

    fn unpack(&self, builder: &mut EntityTransformer) {
        let ent = builder.main_entity();

        // access
        let mut ent_mut = builder.entity_mut(ent);
        let mut tags = ent_mut.get_mut::<Pack>().unwrap();

        // get type-specified tags
        let sprite_path: String = tags.take("sprite_path").unwrap().into();
        let pages:           u8 = tags.take("pages").unwrap().into();
        let durability:      u8 = tags.take("durability").unwrap().into();
        let hp:             u16 = tags.take("hp").unwrap().into();

        let asset_server = builder.world().resource::<AssetServer>().clone();
        let sprite: Handle<Image>   = asset_server.load(&sprite_path);

        // apply commands
        builder.world()
            .commands()
            .entity(ent)
            .insert((Block { pages, durability, hp }, sprite));
    }

    fn pack(&self, builder: &mut EntityTransformer) {
        let ent  = builder.main_entity() ;
        let world = builder.world();
        let asset_server = world.resource::<AssetServer>().clone();

        let mut ent_mut = world.entity_mut(ent);

        let block = *ent_mut.get::<Block>().unwrap();
        let img_handle = ent_mut.get::<Handle<Image>>()
            .unwrap()
            .clone_weak();

        let path = asset_server.get_path(&img_handle).unwrap().to_string();

        // insert tags
        ent_mut.get_mut::<Pack>()
            .unwrap()
            .insert("sprite_path", PackTag::String(path))
            .insert("pages", PackTag::U8(block.pages))
            .insert("durability", PackTag::U8(block.durability))
            .insert("hp", PackTag::U16(block.hp));

        // apply commands
        builder.world()
            .commands()
            .entity(ent)
            .remove::<Block>()
            .remove::<Handle<Image>>();
    }
}

#[derive(Default, Debug)]
pub struct NamePackAttribute;

impl GeneralTransformer for NamePackAttribute {
    fn attribute(&self) -> (&str, TagType) {
        ("name", TagType::String)
    }

    fn unpack(&self, builder: &mut EntityTransformer) {
        let mut ent = builder.main_entity_mut();
        let tag = ent.get_mut::<Pack>().unwrap().take("name")
            .unwrap();

        let name: String = tag.into();

        let ent = builder.main_entity();
        builder.commands().entity(ent)
            .insert(Name::new(name));
    }

    fn pack(&self, builder: &mut EntityTransformer) {
        let mut ent = builder.main_entity_mut();

        if !ent.contains::<Name>() {
            return;
        }

        let name = ent.get::<Name>().unwrap().clone();


        ent.get_mut::<Pack>().unwrap()
            .insert("name", PackTag::String(name.as_str().into()));

        let ent = builder.main_entity();
        builder.commands().entity(ent)
            .remove::<Name>();
    }
}


fn main() {
    App::new()
        .add_plugins(EnginePlugin::new("test"))
        .add_plugins((
            WorldInspectorPlugin::new(),
            StateInspectorPlugin::<AppState>::new(),
        ))
        .register_type::<Block>()
        // transformers
        .add_pack(BlockPack)
        .add_pack_attribute(NamePackAttribute)
        // systems
        .add_systems(PostStartup, style)
        .add_systems(Startup, setup)
        .add_systems(Update, handle_packing)
        .run();
}

fn setup(mut commands: Commands) {
    // spawn camera
    commands.spawn(Camera2dBundle::new_with_far(10.0));

    // spawn tagged entity
    let tags = Pack::new("block")
        .with("sprite_path", PackTag::String("tests/sprite.png".into()))
        .with("pages", PackTag::U8(10))
        .with("durability", PackTag::U8(1))
        .with("hp", PackTag::U16(1))
        .with("name", PackTag::String("Temple wall".into()));

    commands.spawn((
        SpriteBundle::default(),
        tags,
        Unpacked
    ));
}
fn style(mut egui: EguiContexts) {
    let ctx =egui.ctx_mut();

    let mut style = (*ctx.style()).clone();
    let mut visuals = style.visuals.clone();

    // Change background color
    // visuals.window_fill = egui::Color32::from_rgb(240, 240, 240);

    // Change edge (stroke) color
    visuals.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, Color32::from_rgb(100, 100, 100));

    visuals.window_shadow = egui::epaint::Shadow {
        offset: default(),
        blur: 5.0,
        spread: 8.0,
        color: Color32::from_rgba_premultiplied(10, 10, 10, 70),
    };

    // Change corner rounding
    visuals.window_rounding = egui::Rounding::same(0.0);
    visuals.widgets.noninteractive.rounding = egui::Rounding::same(4.0);

    // Apply the modified visuals
    style.visuals = visuals;
    ctx.set_style(style);

}

fn handle_packing(
    mut text_buf: Local<String>,
    mut egui_contexts: EguiContexts,
    mut commands: Commands
) {
    let Some(ctx) = egui_contexts.try_ctx_mut() else {
        return;
    };

    egui::Window::new("Commands").show(ctx, |ui| {
        ui.set_max_width(20.0);

        ui.horizontal(|ui| {
            ui.button("unpack").clicked()
                .then(|| commands.add(UnpackEntities));
            ui.button("pack").clicked()
                .then(|| commands.add(PackEntities));
        });

        ui.separator();

        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut *text_buf);

            ui.button("save").clicked()
                .then(|| {});

            ui.button("Load").clicked()
                .then(|| {});
        });
    });
}

