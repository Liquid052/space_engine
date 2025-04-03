use bevy::ecs::world::Command;
use bevy::prelude::*;
use soa_rs::{soa, Soars};
use test_space::plugins::multi_threaded::MultithreadedSpacePlugin;
use test_space::prelude::*;

fn main() {


}

#[derive(Soars, PartialEq, Debug)]
#[soa_derive(Debug, PartialEq)]
struct Baz {
    foo: u16,
    bar: u8,
}


fn setup(mut commands: Commands) {


    let mut soa = soa![
        Baz { foo: 1, bar: 2 },
        Baz { foo: 3, bar: 4 },
    ];


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
//
//
// #[derive(Default)]
// pub struct SpawnPlanet(pub usize);
//
// impl Command for SpawnPlanet {
//     fn apply(self, world: &mut World) {
//         let parent = world.resource_ref::<StarSystem>().get_sun()
//             .expect("Can't place planet without Sun");
//
//         let keplerian = Keplerian {
//             semi_major_axis: 23_599_840_256.0,
//             ..default()
//         };
//         let name = Name::new(format!("Planet {}", self.0));
//         let depth = SpaceDepth(1);
//         let orbit = Orbit::new(parent);
//         let body = Body::new(600_000.0, 5.2915158e22);
//
//         let ent = world.commands().spawn((
//             name,
//             body,
//             SpacePos::default(),
//             RefFrame::new(),
//             keplerian,
//             StateVec::default(),
//             orbit,
//             // markers
//             depth,
//             StarMarker,
//             SpaceLayer
//         )).id();
//
//         world.get_mut::<RefFrame>(parent).unwrap().push_body(ent);
//     }
// }