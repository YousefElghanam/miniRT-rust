use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::constants::SCENE_FILE_EXTENSION;
use crate::elements::{Light, Material, Mesh, ObjType, Object, Plane, Point, Sphere, Triangle};
use crate::error::MiniRtErr;
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

fn parse_obj_line(words: &[&str], base_dir: &Path, scene: &mut Scene) -> Result<(), MiniRtErr> {
    if !(2..=5).contains(&words.len()) {
        return Err(MiniRtErr::Parse(
            "OBJ expects a path, with optional position, scale, and color".to_string(),
        ));
    }

    let obj_path = base_dir.join(words[1]);
    let position = words
        .get(2)
        .map(|value| parse_vec3(value))
        .transpose()?
        .unwrap_or(Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });
    let scale = words
        .get(3)
        .map(|value| value.parse::<f32>())
        .transpose()
        .map_err(|error| MiniRtErr::Parse(format!("invalid OBJ scale: {error}")))?
        .unwrap_or(1.0);
    let color = words
        .get(4)
        .map(|value| parse_color(value))
        .transpose()?
        .unwrap_or(Color {
            r: 200,
            g: 200,
            b: 200,
        });

    let mesh = load_obj_mesh(&obj_path, position, scale, color)?;
    scene.objects.push(Box::new(mesh));
    Ok(())
}

fn parse_vec3(value: &str) -> Result<Vec3, MiniRtErr> {
    let values: Vec<f32> = value
        .split(',')
        .map(|component| {
            component
                .parse()
                .map_err(|error| MiniRtErr::Parse(format!("invalid vector '{value}': {error}")))
        })
        .collect::<Result<_, _>>()?;
    if values.len() != 3 {
        return Err(MiniRtErr::Parse(format!(
            "expected three vector components: '{value}'"
        )));
    }
    Ok(Vec3 {
        x: values[0],
        y: values[1],
        z: values[2],
    })
}

fn parse_color(value: &str) -> Result<Color, MiniRtErr> {
    let vector = parse_vec3(value)?;
    Ok(Color {
        r: vector.x.clamp(0.0, 255.0) as u8,
        g: vector.y.clamp(0.0, 255.0) as u8,
        b: vector.z.clamp(0.0, 255.0) as u8,
    })
}

pub fn load_obj_mesh(
    path: &Path,
    position: Vec3,
    scale: f32,
    color: Color,
) -> Result<Mesh, MiniRtErr> {
    let (models, _) = tobj::load_obj(
        path,
        &tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
    )
    .map_err(|error| {
        MiniRtErr::Parse(format!("failed to load OBJ '{}': {error}", path.display()))
    })?;

    let mut triangles = Vec::new();
    for model in models {
        let mesh = model.mesh;
        for indices in mesh.indices.chunks_exact(3) {
            let vertices: [Vec3; 3] = indices
                .iter()
                .map(|&index| {
                    let offset = index as usize * 3;
                    Vec3 {
                        x: mesh.positions[offset] * scale + position.x,
                        y: mesh.positions[offset + 1] * scale + position.y,
                        z: mesh.positions[offset + 2] * scale + position.z,
                    }
                })
                .collect::<Vec<_>>()
                .try_into()
                .map_err(|_| {
                    MiniRtErr::Parse("OBJ face did not contain exactly three indices".to_string())
                })?;
            triangles.push(Triangle {
                vertices,
                material: Material { color },
            });
        }
    }

    Mesh::new(triangles).ok_or_else(|| {
        MiniRtErr::Parse(format!(
            "OBJ '{}' does not contain any triangles",
            path.display()
        ))
    })
}

pub fn parse_scene_file(path: &str) -> Result<Scene, MiniRtErr> {
    if path.ends_with(".obj") {
        let mesh = load_obj_mesh(
            Path::new(path),
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            1.0,
            Color {
                r: 200,
                g: 200,
                b: 200,
            },
        )?;
        let mut scene = Scene::default();
        scene.objects.push(Box::new(mesh));
        return build_scene(scene);
    }
    if !path.ends_with(SCENE_FILE_EXTENSION) {
        return Err(MiniRtErr::InvalidSceneFileExtension);
    }
    let mut scene: Scene = Scene::default();
    match File::open(path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let base_dir = Path::new(path).parent().unwrap_or_else(|| Path::new(""));
            for line in reader.lines() {
                let mut line = line?.trim().to_string();
                if let Some(pos) = line.find('#') {
                    line.truncate(pos);
                }
                if line.len() == 0 {
                    continue;
                }
                let words: Vec<&str> = line.split_whitespace().collect();
                if words.first() == Some(&"OBJ") {
                    parse_obj_line(&words, base_dir, &mut scene)?;
                } else {
                    parse_line(&line, &mut scene);
                }
            }
            build_scene(scene)
        }
        Err(_) => Err(MiniRtErr::InvalidSceneFile),
    }
}

fn build_scene(mut scene: Scene) -> Result<Scene, MiniRtErr> {
    scene.rebuild_acceleration();
    Ok(scene)
}
