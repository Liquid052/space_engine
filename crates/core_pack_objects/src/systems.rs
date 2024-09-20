// pub(crate) fn reconstruct_new(
//     mut query: Query<(Entity, &mut TagItem), Added<FromDisk>>,
//     server: Res<AssetServer>,
//     res: Res<MetaReconstructor>,
//     com: ParallelCommands,
// ) {
//     query.par_iter_mut().for_each(|(ent, mut tag_item)| {
//         com.command_scope(|com| {
//             res.load(ent, com, server.as_ref(), tag_item.as_mut());
//         });
//     });
// }
//
// pub(crate) fn cleanup(query: Query<Entity, With<FromDisk>>, com: ParallelCommands) {
//     query.par_iter().for_each(|ent| {
//         com.command_scope(|mut com| {
//             com.entity(ent).remove::<FromDisk>();
//         });
//     });
// }
//
// pub(crate) fn flatten_scenes(
//     mut com: Commands,
//     query: Query<(Entity, &Handle<DynamicScene>)>,
//     server: Res<AssetServer>,
// ) {
//     query.iter().for_each(|(ent, handle)| {
//         let Some(handle) = server.get_load_state(handle) else {
//             return;
//         };
//         if handle != LoadState::Loaded {
//             return;
//         }
//
//         com.entity(ent).clear_children().despawn();
//     });
// }
