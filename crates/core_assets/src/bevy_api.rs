use bevy::app::App;

pub trait BevyItemApi {
    fn add_script_object<T>(&mut self) -> &mut Self;
}

impl BevyItemApi for App {
    fn add_script_object<T>(&mut self) -> &mut Self {
        todo!()
    }
}
