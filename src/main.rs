mod camera;
mod integrators;
mod math;
mod object;
mod sampler;
mod scene;
mod shaders;

use crate::scene::Scene;

fn main() {
    let scene = Scene::new();
    let samples = scene.capture(100u32);
}
