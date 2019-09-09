use crate::camera::{Camera, CameraConstructor};
use crate::materials::{Dielectric, Lambertian, Material, Metal};
use crate::{Ray, Vector};

use super::{HitRecord, Hittable, MovingSphere, Sphere};
use crate::aabb::Aabb;
use crate::bvh::Bvh;

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct Scene {
    objects: Vec<Box<dyn Hittable>>,
    pub camera: Camera,
}

impl Scene {
    pub fn from_objects(
        objects: Vec<Box<dyn Hittable>>,
        aspect_r: f32,
    ) -> Self {
        Scene {
            objects,
            camera: Camera::default(aspect_r),
        }
    }

    pub fn from_objects_and_cam(
        objects: Vec<Box<dyn Hittable>>,
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
    match scene.camera {
        Some(c) => {
            let look_from =
                Vector::new(c.look_from.x, c.look_from.y, c.look_from.z);
            let look_at = Vector::new(c.look_at.x, c.look_at.y, c.look_at.z);

            let focus_dist = c
                .focus_dist
                .unwrap_or_else(|| (look_from - look_at).length());

            let aperture = c.aperture.unwrap_or(0.0001);

            let t0 = c.t0.unwrap_or(0.0);
            let t1 = c.t1.unwrap_or(0.0);

            let objects = parse_objects(scene.objects, t0, t1);

            let camera_settings = CameraConstructor {
                look_from,
                look_at,
                vup: Vector::new(c.vup.x, c.vup.y, c.vup.z),
                vfov: c.vfov,
                aspect_r,
                aperture,
                focus_dist,
                t0,
                t1,
            };

            let camera = Camera::new(camera_settings);

            Scene::from_objects_and_cam(objects, camera)
        }
        None => Scene::from_objects(
            parse_objects(scene.objects, 0.0, 0.0),
            aspect_r,
        ),
    }
}

fn parse_objects(
    scene_objects: Vec<SchemaObject>,
    t0: f32,
    t1: f32,
) -> Vec<Box<dyn Hittable>> {
    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

    for object in scene_objects {
        if object.name == "BVH" {
            objects.push(Bvh::new(
                &mut parse_objects(object.items.unwrap(), t0, t1),
                t0,
                t1,
            ));
            continue;
        }

        let object_material = object.material.unwrap();
        let material: Box<dyn Material> = match object_material.name.as_str() {
            "Metal" => {
                let albedo = object_material.albedo.unwrap();
                let fuzz = object_material.fuzz.unwrap();
                Box::new(Metal::new(albedo.x, albedo.y, albedo.z, fuzz))
            }
            "Lambertian" => {
                let albedo = object_material.albedo.unwrap();
                Box::new(Lambertian::new(albedo.x, albedo.y, albedo.z))
            }
            "Dielectric" => {
                let ref_idx = object_material.ref_idx.unwrap();
                Box::new(Dielectric::new(ref_idx))
            }
            _ => {
                unreachable!(
                    "Unrecognized material type encountered: {}",
                    object_material.name
                );
            }
        };

        match object.name.as_str() {
            "Sphere" => {
                let center = object.center.unwrap();
                let radius = object.radius.unwrap();

                objects.push(Box::new(Sphere::new(
                    Vector::new(center.x, center.y, center.z),
                    radius,
                    material,
                )));
            }
            "MovingSphere" => {
                let center = object.center.unwrap();
                let center2 = object.center2.unwrap();
                let radius = object.radius.unwrap();
                let t0 = object.t0.unwrap();
                let t1 = object.t1.unwrap();

                objects.push(Box::new(MovingSphere::new(
                    Vector::new(center.x, center.y, center.z),
                    Vector::new(center2.x, center2.y, center2.z),
                    t0,
                    t1,
                    radius,
                    material,
                )));
            }
            _ => {
                unreachable!("Unknown object type found");
            }
        }
    }

    objects
}

impl Hittable for Scene {
    fn hit(
        &self,
        r: Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<(HitRecord, &Box<dyn Material>)> {
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

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        if self.objects.is_empty() {
            return None;
        }

        if let Some(temp_box) = self.objects[0].bounding_box(t0, t1) {
            let mut ret = temp_box;

            for item in &self.objects[1..] {
                if let Some(b) = item.bounding_box(t0, t1) {
                    ret = Aabb::surrounding_box(ret, b);
                } else {
                    return None;
                }
            }

            return Some(ret);
        }

        None
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
    center2: Option<SchemaVector>,
    radius: Option<f32>,
    t0: Option<f32>,
    t1: Option<f32>,
    material: Option<SchemaMaterial>,
    items: Option<Vec<SchemaObject>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SchemaCamera {
    look_from: SchemaVector,
    look_at: SchemaVector,
    vup: SchemaVector,
    vfov: f32,
    aperture: Option<f32>,
    focus_dist: Option<f32>,
    t0: Option<f32>,
    t1: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SchemaScene {
    objects: Vec<SchemaObject>,
    camera: Option<SchemaCamera>,
}
