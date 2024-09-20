#![allow(dead_code)]
use bevy::app::App;

pub trait BevyItemApi {
    fn process_item<T>(&mut self) -> &mut Self;
}

impl BevyItemApi for App {
    fn process_item<T>(&mut self) -> &mut Self {
        todo!()
    }
}
