use std::env;

mod app;
mod camera;
mod constants;
mod elements;
mod error;
mod input;
mod maths;
mod parse;
mod render;
mod renderer;
mod scene;

use crate::error::MiniRtErr;
use crate::parse::parse_scene_file;
use crate::scene::Scene;

fn log_scene_content(scene: &Scene) {
    // let _ = scene.cones.iter().map(|cone| print!("{cone:?}"));
    // let _ = scene.cylinders.iter().map(|cone| print!("{cone:?}"));
    // let _ = scene.planes.iter().map(|cone| print!("{cone:?}"));
    // println!("Spheres:");
    // for sphere in &scene.spheres {
    //     println!("{sphere:?}");
    // }
    // print!("\nLights:");
    // for light in &scene.lights {
    //     println!("{light:?}");
    // }
    for object in &scene.objects {
        println!("{object:?}");
    }
}

fn main() -> Result<(), MiniRtErr> {
    let args: Vec<String> = env::args().collect();
    let scene = match args.as_slice() {
        [_] => Scene::default(),
        [_, path] => {
            print!("Reading scene file... ");
            let scene = parse_scene_file(path)?;
            println!("Success");
            scene
        }
        _ => return Err(MiniRtErr::InvalidNumberOfArguments),
    };
    log_scene_content(&scene);
    render::render(scene);
    Ok(())
}
