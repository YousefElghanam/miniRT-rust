use std::env;

mod app;
mod camera;
mod constants;
mod elements;
mod error;
mod event_loop;
mod input;
mod maths;
mod parse;
mod player;
mod renderer;
mod scene;

use crate::error::MiniRtErr;
use crate::parse::parse_scene_file;
use crate::scene::Scene;

fn log_scene_content(scene: &Scene) {
    print!("\nLights:");
    for light in &scene.lights {
        println!("{light:?}");
    }
    print!("\nObjects:");
    for object in &scene.objects {
        println!("{object:?}");
    }
}

fn main() -> Result<(), MiniRtErr> {
    let args: Vec<String> = env::args().collect();
    let mut scene = match args.as_slice() {
        [_] => Scene::default(),
        [_, path] => {
            print!("Reading scene file... ");
            parse_scene_file(path)?
        }
        _ => return Err(MiniRtErr::InvalidNumberOfArguments),
    };
    log_scene_content(&scene);
    scene.build_scene();
    event_loop::render(scene);
    Ok(())
}
