use crate::materials::{Lambertian, Material, Metal};
use crate::{Ray, Vector};

use super::{HitRecord, Hittable, Sphere};

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct World {
    objects: Vec<Box<Hittable>>,
}

impl World {
    pub fn from_objects(objects: Vec<Box<Hittable>>) -> Self {
        World { objects }
    }

    pub fn from_json(path: &Path) -> Result<Self, String> {
        let json = fs::read_to_string(path);

        match json {
            Ok(content) => {
                let world = serde_json::from_str::<SchemaWorld>(&content);

                match world {
                    Ok(world) => Ok(schema_world_to_world(world)),
                    Err(e) => Err(format!("Failed to parse JSON: {}", e)),
                }
            }
            Err(_) => Err(String::from("Failed to read file")),
        }
    }
}

fn schema_world_to_world(s_world: SchemaWorld) -> World {
    let mut objects: Vec<Box<Hittable>> = Vec::new();

    for object in s_world.objects {
        match object.name.as_str() {
            "Sphere" => {
                let center = object.center.unwrap();
                let radius = object.radius.unwrap();
                match object.material.name.as_str() {
                    "Metal" => {
                        let albedo = object.material.albedo;
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
                        let albedo = object.material.albedo;

                        objects.push(Box::new(Sphere::new(
                            Vector::new(center.x, center.y, center.z),
                            radius,
                            Box::new(Lambertian::new(
                                albedo.x, albedo.y, albedo.z,
                            )),
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

    return World::from_objects(objects);
}

impl Hittable for World {
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
/*     AUTO GENERATED FROM SCHEMA -- DO NOT MODIFY      */
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
    albedo: SchemaVector,
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
struct SchemaWorld {
    objects: Vec<SchemaObject>,
}
