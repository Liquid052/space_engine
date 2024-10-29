use bevy::ecs::world::Command;
use bevy::prelude::*;
use test_space::plugins::single_threaded::AOSSpacePlugin;
use test_space::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(MinimalPlugins)
        .add_plugins(AOSSpacePlugin)
        .add_systems(Startup, setup_aos);

    app.update();
}

fn setup_aos(mut commands: Commands) {
    let star = commands.spawn((
        Name::new("Star"),
        AOSBody {
            body: Body {
                radius: 261_600_000.0,
                mass: 1.7565459e28,
                ..default()
            },
            space_pos: SpacePos::default(),
            ref_frame: RefFrame::new(),
            keplerian: Keplerian::default(),
            state_vectors: StateVec::default(),
            space_depth: SpaceDepth(0),
            orbit: Orbit::default(), // Star doesn't orbit anything
        },
        StarMarker,
        SpaceLayer
    )).id();

    commands.insert_resource(StarSystem {
        system_name: "test".into(),
        sun: Some(star),
    });

    for x in 0..=100_000 {
        commands.add(SpawnPlanetAoS(x));
    }
}

#[derive(Default)]
pub struct SpawnPlanetAoS(pub usize);

impl Command for SpawnPlanetAoS {
    fn apply(self, world: &mut World) {
        let parent = world.resource_ref::<StarSystem>().get_sun()
            .expect("Can't place planet without Sun");

        let keplerian = Keplerian {
            semi_major_axis: 23_599_840_256.0,
            ..default()
        };

        let aos_body = AOSBody {
            body: Body::new(600_000.0, 5.2915158e22),
            space_pos: SpacePos::default(),
            ref_frame: RefFrame::new(),
            keplerian,
            state_vectors: StateVec::default(),
            orbit: Orbit::new(parent),
            space_depth: SpaceDepth(1),
        };

        let ent = world.spawn((
            Name::new(format!("Planet {}", self.0)),
            aos_body,
            SpaceLayer
        )).id();

        // Update parent's RefFrame
        if let Some(mut parent_body) = world.get_mut::<AOSBody>(parent) {
            parent_body.ref_frame.push_body(ent);
        }
    }
}