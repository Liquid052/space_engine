use bevy::ecs::world::Command;
use bevy::prelude::*;
use test_space::plugins::multi_threaded::MultithreadedSpacePlugin;
use test_space::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(MinimalPlugins)
        .add_plugins(MultithreadedSpacePlugin)
        .add_systems(Startup, setup)
        .run();

    app.update();
}

fn setup(mut commands: Commands) {
    let star = commands.spawn((
        Name::new("Star"),
        Body {
            radius: 261_600_000.0,
            mass: 1.7565459e28,
            ..default()
        },
        SpacePos::default(),
        RefFrame::new(),
        Keplerian::default(),
        StateVec::default(),
        // markers
        SpaceDepth(0),
        StarMarker,
        SpaceLayer
    )).id();

    commands.insert_resource(StarSystem {
        system_name: "test".into(),
        sun: Some(star),
    });

    for x in 0..=100_000 {
        commands.add(SpawnPlanet(x));
    }
}


#[derive(Default)]
pub struct SpawnPlanet(pub usize);

impl Command for SpawnPlanet {
    fn apply(self, world: &mut World) {
        let parent = world.resource_ref::<StarSystem>().get_sun()
            .expect("Can't place planet without Sun");

        let keplerian = Keplerian {
            semi_major_axis: 23_599_840_256.0,
            ..default()
        };
        let name = Name::new(format!("Planet {}", self.0));
        let depth = SpaceDepth(1);
        let orbit = Orbit::new(parent);
        let body = Body::new(600_000.0, 5.2915158e22);

        let ent = world.commands().spawn((
            name,
            body,
            SpacePos::default(),
            RefFrame::new(),
            keplerian,
            StateVec::default(),
            orbit,
            // markers
            depth,
            StarMarker,
            SpaceLayer
        )).id();

        world.get_mut::<RefFrame>(parent).unwrap().push_body(ent);
    }
}