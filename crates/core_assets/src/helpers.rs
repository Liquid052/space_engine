use std::fs::{File, remove_file};
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

use walkdir::WalkDir;

static PREPROCESSED: AtomicBool = AtomicBool::new(false);

pub fn generate_mod_index() {
    if PREPROCESSED.load(Ordering::SeqCst) {
        return;
    }

    let matching_dirs: Vec<_> = WalkDir::new(".")
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.file_type().is_dir() &&
                entry.file_name().to_str() == Some("mods")
        })
        .map(|entry| entry.path().to_path_buf())
        .collect();

    // process each root with mods directory - required because IDE may start cargo from different directory and thus be inconsistent
    for dir in matching_dirs.iter() {
        let result = get_index_file(dir);
        let Ok(file) = result else {
            panic!("index file un-obtained")
        };

        // println!("CURRENT_MOD_DIR: {}", dir.display());

        scan_mods(file, dir);
    }

    PREPROCESSED.store(true, Ordering::SeqCst);
}

// helpers
fn get_index_file(dir: &PathBuf) -> io::Result<File> {
    let index_path = dir.join("paths.mod_index.ron");

    if index_path.exists() {
        remove_file(index_path.clone()).unwrap();
    }

    File::create(index_path)
}

fn scan_mods(mut file: File, path_buf: &PathBuf) {
    const MOD_FILENAME: &'static str = "mod.mod_info.ron";
    let mut text = "( paths: [".to_string();
    let path = path_buf.as_path().to_str().unwrap();

    let mut contains = false;
    WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        // filter only those being subdirectories of mod index and remap
        .filter_map(|entry| {
            let is_dir = entry.file_type().is_dir();
            let is_subdirectory = path_depth_diff(path_buf.as_path(), entry.path()) == 1;
            if !(is_dir && is_subdirectory) {
                return None;
            }

            let mut path = entry.clone().into_path();
            path.push(MOD_FILENAME);

            // println!("X>: {}", path.display());

            match path.as_path().exists() {
                true => Some(path),
                false => None
            }
        })
        .for_each(|mod_dir| {
            let path = remove_parent_directories_until_assets(mod_dir.as_path())
                .unwrap();

            let txt = format!(" \"{}\",", path.as_path().display());

            contains = true;
            text.push_str(&txt);
        });

    if contains {
        text.remove(text.len() - 1);
    }
    text.push_str("])");
    let text = text.replace("\\", "\\\\");

    file.write(text.as_bytes()).unwrap();
}

fn path_depth_diff(path1: &Path, path2: &Path) -> usize {
    let depth1 = path1.components().count();
    let depth2 = path2.components().count();

    if depth1 > depth2 {
        depth1 - depth2
    } else {
        depth2 - depth1
    }
}

fn remove_parent_directories_until_assets(path: &Path) -> Option<PathBuf> {
    // Iterate through the components of the path
    for (i, component) in path.iter().enumerate() {
        // Check if the current component is "assets"
        if component == "assets" {
            // Rebuild the path from "assets" onward
            return Some(path.iter().skip(i + 1).collect());
        }
    }

    // Return None if "assets" is not found
    None
}