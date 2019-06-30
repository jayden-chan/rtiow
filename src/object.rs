use crate::{Ray, Vector};

#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    pub t: Option<f32>,
    pub p: Option<Vector>,
    pub normal: Option<Vector>,
}

impl HitRecord {
    pub fn empty() -> Self {
        Self {
            t: None,
            p: None,
            normal: None,
        }
    }
}

pub trait Object {
    fn hit(
        self,
        r: Ray,
        t_min: f32,
        t_max: f32,
        hit_record: &mut HitRecord,
    ) -> bool;
}
