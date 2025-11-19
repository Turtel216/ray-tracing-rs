use crate::vec::{Point3, Vec3};

#[derive(Default)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
        }
    }

    #[inline]
    pub fn origin(&self) -> Point3 {
        self.orig
    }

    #[inline]
    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    #[inline]
    pub fn at(&self, t: f32) -> Point3 {
        self.orig + t * self.dir
    }
}
