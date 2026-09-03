use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::constants::SCENE_FILE_EXTENSION;
use crate::error::MiniRtErr;
use crate::scene::{ObjType, Object, Point, Scene, Vec3};

pub fn generate_default_scene() -> Scene {
    unimplemented!("generating a default scene"); // TODO
}

// pub fn parse_point(word: &str) -> Point {
//     let nums = word.split()
// }

pub fn parse_sphere_line(words: &Vec<&str>) -> Object {
    if words.len() != 4 {
        panic!("invalid number of words for SP");
    }
    let position_axes: Vec<&str> = words[2].split(',').collect();
    if position_axes.len() != 3 {
        panic!("invalid number of arguments for SP Position");
    }
    let direction_axes: Vec<&str> = words[3].split(',').collect();
    if direction_axes.len() != 3 {
        panic!("invalid number of arguments for SP Direction");
    }
    let obj = Object {
        obj_type: ObjType::SPHERE,
        scale: words[1].parse().unwrap_or(1),
        position: Point {
            x: position_axes[0].parse().unwrap_or(0.0),
            y: position_axes[1].parse().unwrap_or(0.0),
            z: position_axes[2].parse().unwrap_or(0.0),
        },
        direction: Vec3 {
            x: direction_axes[0].parse().unwrap_or(0.0),
            y: direction_axes[1].parse().unwrap_or(0.0),
            z: direction_axes[2].parse().unwrap_or(0.0),
        },
    };
    obj
}

pub fn parse_cylinder_line(words: &Vec<&str>) -> Object {
    if words.len() != 4 {
        panic!("invalid number of words for CY");
    }
    let position_axes: Vec<&str> = words[2].split(',').collect();
    if position_axes.len() != 3 {
        panic!("invalid number of arguments for CY Position");
    }
    let direction_axes: Vec<&str> = words[3].split(',').collect();
    if direction_axes.len() != 3 {
        panic!("invalid number of arguments for CY Direction");
    }
    let obj = Object {
        obj_type: ObjType::CYLINDER,
        scale: words[1].parse().unwrap_or(1),
        position: Point {
            x: position_axes[0].parse().unwrap_or(0.0),
            y: position_axes[1].parse().unwrap_or(0.0),
            z: position_axes[2].parse().unwrap_or(0.0),
        },
        direction: Vec3 {
            x: direction_axes[0].parse().unwrap_or(0.0),
            y: direction_axes[1].parse().unwrap_or(0.0),
            z: direction_axes[2].parse().unwrap_or(0.0),
        },
    };
    obj
}

pub fn parse_plane_line(words: &Vec<&str>) -> Object {
    if words.len() != 4 {
        panic!("invalid number of words for PL");
    }
    let position_axes: Vec<&str> = words[2].split(',').collect();
    if position_axes.len() != 3 {
        panic!("invalid number of arguments for PL Position");
    }
    let direction_axes: Vec<&str> = words[3].split(',').collect();
    if direction_axes.len() != 3 {
        panic!("invalid number of arguments for PL Direction");
    }
    let obj = Object {
        obj_type: ObjType::PLANE,
        scale: words[1].parse().unwrap_or(1),
        position: Point {
            x: position_axes[0].parse().unwrap_or(0.0),
            y: position_axes[1].parse().unwrap_or(0.0),
            z: position_axes[2].parse().unwrap_or(0.0),
        },
        direction: Vec3 {
            x: direction_axes[0].parse().unwrap_or(0.0),
            y: direction_axes[1].parse().unwrap_or(0.0),
            z: direction_axes[2].parse().unwrap_or(0.0),
        },
    };
    obj
}

pub fn parse_cone_line(words: &Vec<&str>) -> Object {
    if words.len() != 4 {
        panic!("invalid number of words for CO");
    }
    let position_axes: Vec<&str> = words[2].split(',').collect();
    if position_axes.len() != 3 {
        panic!("invalid number of arguments for CO Position");
    }
    let direction_axes: Vec<&str> = words[3].split(',').collect();
    if direction_axes.len() != 3 {
        panic!("invalid number of arguments for CO Direction");
    }
    let obj = Object {
        obj_type: ObjType::CONE,
        scale: words[1].parse().unwrap_or(1),
        position: Point {
            x: position_axes[0].parse().unwrap_or(0.0),
            y: position_axes[1].parse().unwrap_or(0.0),
            z: position_axes[2].parse().unwrap_or(0.0),
        },
        direction: Vec3 {
            x: direction_axes[0].parse().unwrap_or(0.0),
            y: direction_axes[1].parse().unwrap_or(0.0),
            z: direction_axes[2].parse().unwrap_or(0.0),
        },
    };
    obj
}

pub fn parse_line(line: String, scene: &mut Scene) {
    let words: Vec<&str> = line.split_whitespace().collect();
    if words[0] == "SP" {
        let sphere = parse_sphere_line(&words);
        scene.spheres.push(sphere);
    }
    if line.starts_with("PL") {
        let plane = parse_plane_line(&words);
        scene.planes.push(plane);
    }
    if line.starts_with("CY") {
        let cylinder = parse_cylinder_line(&words);
        scene.cylinders.push(cylinder);
    }
    if line.starts_with("CO") {
        let cone = parse_cone_line(&words);
        scene.cones.push(cone);
    }
}

pub fn parse_scene_file() -> Result<Scene, MiniRtErr> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 && !&args[1].ends_with(SCENE_FILE_EXTENSION) {
        return Err(MiniRtErr::InvalidSceneFileExtension);
    }
    let mut scene: Scene = Scene::default();
    match File::open(&args[1]) {
        Ok(file) => {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let mut line = line.unwrap_or_else(|_| "".to_string()).trim().to_string();
                if let Some(pos) = line.find('#') {
                    line.truncate(pos);
                }
                if line.len() == 0 {
                    continue;
                }
                parse_line(line, &mut scene);
            }
            return Ok(scene);
        }
        Err(_) => Err(MiniRtErr::InvalidSceneFile),
    }
}
