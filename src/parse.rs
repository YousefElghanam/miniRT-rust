use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::constants::SCENE_FILE_EXTENSION;
use crate::elements::{Light, Material, ObjType, Object, Plane, Point, Sphere};
use crate::error::MiniRtErr;
use crate::maths::build_bvh;
use crate::maths::{Color, Vec3};
use crate::scene::Scene;

pub fn generate_default_scene() -> Scene {
    unimplemented!("generating a default scene"); // TODO
}

// pub fn parse_point(word: &str) -> Point {
//     let nums = word.split()
// }

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

pub fn parse_sphere_line(words: &Vec<&str>) -> Sphere {
    // TODO validate color boundaries. show how to use arguments for each element

    if words.len() != 4 {
        panic!("invalid number of words for SP");
    }
    let position_axes: Vec<&str> = words[2].split(',').collect();
    if position_axes.len() != 3 {
        panic!("invalid number of arguments for SP Position");
    }
    let colors: Vec<&str> = words[3].split(',').collect();
    if colors.len() != 3 {
        panic!("invalid number of arguments for SP Color");
    }
    Sphere {
        radius: words[1].parse().unwrap_or(1.0),
        center: Vec3 {
            x: position_axes[0].parse().unwrap_or(0.0),
            y: position_axes[1].parse().unwrap_or(0.0),
            z: position_axes[2].parse().unwrap_or(0.0),
        },
        material: Material {
            color: Color {
                r: colors[0].parse().unwrap_or(0),
                g: colors[1].parse().unwrap_or(0),
                b: colors[2].parse().unwrap_or(0),
            },
        },
    }
}

pub fn parse_plane_line(words: &Vec<&str>) -> Plane {
    if words.len() != 4 {
        panic!("invalid number of words for PL");
    }
    let point_axes: Vec<&str> = words[1].split(',').collect();
    if point_axes.len() != 3 {
        panic!("invalid number of arguments for PL Position");
    }
    let normal_axes: Vec<&str> = words[2].split(',').collect();
    if normal_axes.len() != 3 {
        panic!("invalid number of arguments for PL Direction");
    }
    let colors: Vec<&str> = words[3].split(',').collect();
    if colors.len() != 3 {
        panic!("invalid number of arguments for PL Color");
    }
    Plane {
        point: Vec3 {
            x: point_axes[0].parse().unwrap_or(0.0),
            y: point_axes[1].parse().unwrap_or(0.0),
            z: point_axes[2].parse().unwrap_or(0.0),
        },
        normal: Vec3 {
            x: normal_axes[0].parse().unwrap_or(0.0),
            y: normal_axes[1].parse().unwrap_or(0.0),
            z: normal_axes[2].parse().unwrap_or(0.0),
        },
        material: Material {
            color: Color {
                r: colors[0].parse().unwrap_or(0),
                g: colors[1].parse().unwrap_or(0),
                b: colors[2].parse().unwrap_or(0),
            },
        },
    }
}

pub fn parse_light_line(words: &Vec<&str>) -> Light {
    if words.len() != 2 {
        panic!("invalid number of words for LIGHT");
    }
    let position_axes: Vec<&str> = words[1].split(',').collect();
    if position_axes.len() != 3 {
        panic!("invalid number of arguments for LIGHT Position");
    }
    Light {
        position: Vec3 {
            x: position_axes[0].parse().unwrap_or(0.0),
            y: position_axes[1].parse().unwrap_or(0.0),
            z: position_axes[2].parse().unwrap_or(0.0),
        },
    }
}

pub fn parse_line(line: &str, scene: &mut Scene) {
    let words: Vec<&str> = line.split_whitespace().collect();
    if words.is_empty() {
        return;
    }
    if words[0] == "LIGHT" {
        let light = parse_light_line(&words);
        scene.lights.push(light);
    }
    if words[0] == "SP" {
        let sphere = parse_sphere_line(&words);
        scene.objects.push(Box::new(sphere));
    }
    if line.starts_with("PL") {
        let plane = parse_plane_line(&words);
        scene.objects.push(Box::new(plane));
    }
    // if line.starts_with("CY") {
    //     let cylinder = parse_cylinder_line(&words);
    //     scene.cylinders.push(cylinder);
    // }
    // if line.starts_with("CO") {
    //     let cone = parse_cone_line(&words);
    //     scene.cones.push(cone);
    // }
}

pub fn parse_scene_file(path: &str) -> Result<Scene, MiniRtErr> {
    if !path.ends_with(SCENE_FILE_EXTENSION) {
        return Err(MiniRtErr::InvalidSceneFileExtension);
    }
    let mut scene: Scene = Scene::default();
    match File::open(path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let mut line = line?.trim().to_string();
                if let Some(pos) = line.find('#') {
                    line.truncate(pos);
                }
                if line.len() == 0 {
                    continue;
                }
                parse_line(&line, &mut scene);
            }
            let mut bounded_objects = Vec::new();
            let mut unbounded_objects = Vec::new();

            for object in scene.objects {
                if object.bounding_box().is_some() {
                    bounded_objects.push(object);
                } else {
                    unbounded_objects.push(object);
                }
            }
            let bvh = if bounded_objects.is_empty() {
                None
            } else {
                Some(build_bvh(bounded_objects))
            };
            return Ok(Scene {
                bvh,
                unbounded_objects,
                objects: Vec::new(),
                lights: scene.lights,
            });
        }
        Err(_) => Err(MiniRtErr::InvalidSceneFile),
    }
}
