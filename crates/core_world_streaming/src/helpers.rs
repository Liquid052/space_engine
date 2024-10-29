mod scene_saver;
mod scene_loader;

pub use scene_loader::*;
pub use scene_saver::*;

const SCENE_TREE_SCENE: &str = "scene_tree.scn.ron";

pub fn format_into_write(path: &str) -> String {
    format!("assets/saves/{}", path)
}

pub fn format_into_load(load_name: &str) -> String {
    format!("saves/{}", load_name)
}

pub fn scene_tree_load(path: &str) -> String {
    let load = format_into_load(path);

    format!("{}/{}", load, SCENE_TREE_SCENE)
}
pub fn scene_tree_write(path: &str) -> String {
    let write = format_into_write(path);

    format!("{}/{}", write, SCENE_TREE_SCENE)
}
