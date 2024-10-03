#![allow(dead_code)]
use bevy::app::App;

pub trait ItemApiExt {
    fn process_item<T>(&mut self) -> &mut Self;
}

impl ItemApiExt for App {
    fn process_item<T>(&mut self) -> &mut Self {
        todo!()
    }
}
