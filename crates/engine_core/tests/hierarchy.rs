// use bevy::prelude::*;
// use enum_map::*;
//
// use engine_core::prelude::*;
//
// // Marker type
// #[derive(Enum, Default)]
// pub enum ChildEnum {
//     #[default]
//     Head,
//     Arms,
// }
//
// #[test]
// fn hierarchy() {
//     let mut app = App::new();
//
//     app.add_plugins(TestPlugin)
//         .add_systems(Startup, spawn_children)
//         .add_systems(PostStartup, check_hierarchy);
//
//     app.update();
// }
//
// fn spawn_children(mut com: Commands) {
//     let mut map = ChildMap::<ChildEnum>::default();
//
//     let head = com.spawn(Name::new("Head")).id();
//     let arms = com.spawn(Name::new("Arms")).id();
//
//     let parent = com
//         .spawn(Name::new("Parent"))
//         .push_children(&[head, arms])
//         .id();
//
//     map.set(ChildEnum::Head, head);
//     map.set(ChildEnum::Arms, arms);
//
//     com.entity(parent).insert(map);
// }
// fn check_hierarchy(
//     parent: Query<(&Name, &ChildMap<ChildEnum>, &Children)>,
//     child_names: Query<(Entity, &Name), Without<Children>>,
// ) {
//     let (name, child_map, children) = parent.single();
//
//     assert_eq!(Name::new("Parent"), name.clone());
//
//     let (ch_ent, name) = child_names
//         .get(
//             child_map
//                 .get(ChildEnum::Head)
//                 .expect("Child of ChildrenEnum::Head not in hierarchy"),
//         )
//         .expect("Entity of ChildrenEnum::Head not found");
//
//     assert!(children.iter().any(|ent| ch_ent == *ent));
//     assert_eq!(Name::new("Head"), name.clone());
//
//     let (ch_ent, name) = child_names
//         .get(
//             child_map
//                 .get(ChildEnum::Arms)
//                 .expect("Child of ChildrenEnum::Arms not in hierarchy"),
//         )
//         .expect("Entity of ChildrenEnum::Arms not found");
//
//     assert!(children.iter().any(|ent| ch_ent == *ent));
//     assert_eq!(Name::new("Arms"), name.clone());
// }
