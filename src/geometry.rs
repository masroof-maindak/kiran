#[derive(Clone, Default)]
pub struct Vec3f {
    r: f32,
    g: f32,
    b: f32,
}

impl Vec3f {
    pub fn new(r: f32, g: f32, b: f32) -> Vec3f {
        Vec3f { r, g, b }
    }
}

impl IntoIterator for Vec3f {
    type Item = f32;
    type IntoIter = Vec3fIterator;

    fn into_iter(self) -> Self::IntoIter {
        Vec3fIterator { pt: self, idx: 0 }
    }
}

pub struct Vec3fIterator {
    pt: Vec3f,
    idx: usize,
}

impl Iterator for Vec3fIterator {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        let ret = match self.idx {
            0 => Some(self.pt.r),
            1 => Some(self.pt.g),
            2 => Some(self.pt.b),
            _ => None,
        };

        self.idx += 1;
        ret
    }
}

pub struct Sphere {
    origin: Vec3f,
    radius: f32,
}

impl Sphere {
    pub fn new(origin: Vec3f, radius: f32) -> Sphere {
        Sphere { origin, radius }
    }

    pub fn _ray_intersect(src: Vec3f, dir: Vec3f) -> bool {
        // doesn't intersect, intersects at a single point, or intersects at 2 points

        // First of all, if the distance b/w the center of the sphere and the ray is greater than
        // the radius of the sphere, then they simply can't be intersecting, at all

        true
    }
}
