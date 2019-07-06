use crate::materials::{Dielectric, Lambertian, Material, Metal};
use crate::Camera;
use crate::{Ray, Vector};

use super::{HitRecord, Hittable, Sphere};

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct Scene {
    objects: Vec<Box<Hittable>>,
    pub camera: Camera,
}

impl Scene {
    pub fn from_objects(objects: Vec<Box<Hittable>>, aspect_r: f32) -> Self {
        Scene {
            objects,
            camera: Camera::default(aspect_r),
        }
    }

    pub fn from_objects_and_cam(
        objects: Vec<Box<Hittable>>,
        camera: Camera,
    ) -> Self {
        Self { objects, camera }
    }

    pub fn from_json(path: &Path, aspect_r: f32) -> Result<Self, String> {
        fs::read_to_string(path)
            .map_err(|e| format!("Failed to read JSON file: {}", e))
            .and_then(|json| {
                serde_json::from_str::<SchemaScene>(&json)
                    .map_err(|e| format!("Failed to parse JSON: {}", e))
                    .and_then(|scene| {
                        Ok(schema_scene_to_scene(scene, aspect_r))
                    })
            })
    }
}

/// Converts a scene grabbed from a JSON file into a real scene
/// usable by the main renderer. The reason this function is necessary
/// is because the JSON schema for the scene files isn't directly
/// translatable to Rust types.
fn schema_scene_to_scene(scene: SchemaScene, aspect_r: f32) -> Scene {
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

    match scene.camera {
        Some(c) => {
            let look_from =
                Vector::new(c.look_from.x, c.look_from.y, c.look_from.z);
            let look_at = Vector::new(c.look_at.x, c.look_at.y, c.look_at.z);

            let focus_dist =
                c.focus_dist.unwrap_or((look_from - look_at).length());

            let aperture = c.aperture.unwrap_or(0.0001);

            let camera = Camera::new(
                look_from,
                look_at,
                Vector::new(c.vup.x, c.vup.y, c.vup.z),
                c.vfov,
                aspect_r,
                aperture,
                focus_dist,
            );

            Scene::from_objects_and_cam(objects, camera)
        }
        None => Scene::from_objects(objects, aspect_r),
    }
}

impl Hittable for Scene {
    fn hit(
        &self,
        r: Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<(HitRecord, &Box<Material>)> {
        let mut result = None;
        let mut closest_so_far = t_max;

        for i in &self.objects {
            if let Some((hr, mat)) = i.hit(r, t_min, closest_so_far) {
                result = Some((hr, mat));
                closest_so_far = hr.t;
            }
        }

        result
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
struct SchemaCamera {
    look_from: SchemaVector,
    look_at: SchemaVector,
    vup: SchemaVector,
    vfov: f32,
    aperture: Option<f32>,
    focus_dist: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SchemaScene {
    objects: Vec<SchemaObject>,
    camera: Option<SchemaCamera>,
}
