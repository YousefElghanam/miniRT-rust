mod camera;
mod constants;
mod error;
mod input;
mod parse;
mod render;
mod scene;

use crate::error::MiniRtErr;
use crate::scene::Scene;

fn log_scene_content(scene: &Scene) {
    print!("Spheres:");
    let _ = scene.cones.iter().map(|cone| print!("{cone:?}"));
    let _ = scene.cylinders.iter().map(|cone| print!("{cone:?}"));
    let _ = scene.planes.iter().map(|cone| print!("{cone:?}"));
    let _ = scene.spheres.iter().map(|cone| print!("{cone:?}"));
    let _ = scene.spot_lights.iter().map(|cone| print!("{cone:?}"));
}

fn main() -> Result<(), MiniRtErr> {
    // let args: Vec<String> = env::args().collect();
    // let mut scene: Scene = Scene::default();
    // if args.len() > 2 || args.len() < 1 {
    //     drop(MiniRtErr::InvalidNumberOfArguments);
    // } else if args.len() == 1 {
    //     println!("Generating default scene");
    //     scene = generate_default_scene();
    // } else {
    //     print!("Reading scene file... ");
    //     match parse_scene_file() {
    //         Ok(res) => {
    //             println!("Success");
    //             scene = res;
    //         }
    //         Err(err) => {
    //             println!("Failed");
    //             return Err(err);
    //         }
    //     }
    // }
    // log_scene_content(&scene);
    render::render();
    Ok(())
}
