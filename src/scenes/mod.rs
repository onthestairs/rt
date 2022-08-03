use crate::{camera::Camera, hittable::Hittable};

pub mod example;
pub mod example_bvh;
pub mod nts;

pub enum SceneConfig {
    Example,
    ExampleBVH,
    NTS,
}

pub struct Scene {
    pub aspect_ratio: f64,
    pub world: Box<dyn Hittable + Send + Sync>,
    pub camera: Camera,
}

pub fn get_scene(scene_config: SceneConfig) -> Scene {
    match scene_config {
        SceneConfig::Example => example::scene(),
        SceneConfig::ExampleBVH => example_bvh::scene(),
        SceneConfig::NTS => nts::scene(),
    }
}
