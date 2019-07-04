use crate::materials::{Dielectric, Lambertian, Material, Metal};
use crate::{Ray, Vector};

use super::{HitRecord, Hittable, Sphere};

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct Scene {
    objects: Vec<Box<Hittable>>,
}

impl Scene {
    pub fn from_objects(objects: Vec<Box<Hittable>>) -> Self {
        Scene { objects }
    }

    pub fn from_json(path: &Path) -> Result<Self, String> {
        let json = fs::read_to_string(path);

        match json {
            Ok(content) => {
                let scene = serde_json::from_str::<SchemaScene>(&content);

                match scene {
                    Ok(scene) => Ok(schema_scene_to_scene(scene)),
                    Err(e) => Err(format!("Failed to parse JSON: {}", e)),
                }
            }
            Err(_) => Err(String::from("Failed to read file")),
        }
    }
}

/// Converts a scene grabbed from a JSON file into a real scene
/// usable by the main renderer. The reason this function is necessary
/// is because the JSON schema for the scene files isn't directly
/// translatable to Rust types.
fn schema_scene_to_scene(scene: SchemaScene) -> Scene {
    let mut objects: Vec<Box<Hittable>> = Vec::new();

    for object in scene.objects {
        match object.name.as_str() {
            "Sphere" => {
                let center = object.center.unwrap();
                let radius = object.radius.unwrap();
                match object.material.name.as_str() {
                    "Metal" => {
                        let albedo = object.material.albedo.unwrap();
                        let fuzz = object.material.fuzz.unwrap();

                        objects.push(Box::new(Sphere::new(
                            Vector::new(center.x, center.y, center.z),
                            radius,
                            Box::new(Metal::new(
                                albedo.x, albedo.y, albedo.z, fuzz,
                            )),
                        )));
                    }
                    "Lambertian" => {
                        let albedo = object.material.albedo.unwrap();

                        objects.push(Box::new(Sphere::new(
                            Vector::new(center.x, center.y, center.z),
                            radius,
                            Box::new(Lambertian::new(
                                albedo.x, albedo.y, albedo.z,
                            )),
                        )));
                    }
                    "Dielectric" => {
                        let ref_idx = object.material.ref_idx.unwrap();

                        objects.push(Box::new(Sphere::new(
                            Vector::new(center.x, center.y, center.z),
                            radius,
                            Box::new(Dielectric::new(ref_idx)),
                        )));
                    }
                    _ => {
                        panic!(
                            "Unrecognized material type encountered: {}",
                            object.material.name
                        );
                    }
                }
            }
            _ => {
                panic!("Unrecognized object type encountered: {}", object.name);
            }
        }
    }

    return Scene::from_objects(objects);
}

impl Hittable for Scene {
    fn hit(
        &self,
        r: Ray,
        t_min: f32,
        t_max: f32,
    ) -> (bool, Option<(HitRecord, &Box<Material>)>) {
        let mut result = None;
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for i in &self.objects {
            let (hit, res) = i.hit(r, t_min, closest_so_far);
            if hit {
                let (hr, mat) = res.unwrap();
                result = Some((hr, mat));
                hit_anything = true;
                closest_so_far = hr.t;
            }
        }

        (hit_anything, result)
    }
}

/********************************************************/
/*              AUTO GENERATED FROM SCHEMA              */
/********************************************************/

#[derive(Debug, Serialize, Deserialize)]
struct SchemaVector {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct SchemaMaterial {
    name: String,
    albedo: Option<SchemaVector>,
    ref_idx: Option<f32>,
    fuzz: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SchemaObject {
    name: String,
    center: Option<SchemaVector>,
    radius: Option<f32>,
    material: SchemaMaterial,
}

#[derive(Debug, Serialize, Deserialize)]
struct SchemaScene {
    objects: Vec<SchemaObject>,
}
