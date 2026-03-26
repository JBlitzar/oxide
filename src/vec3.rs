
#[derive(Copy, Clone,Debug, PartialEq, PartialOrd)]
pub struct Vec3 {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
}


impl Vec3 {

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    #[inline(always)]
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    #[inline(always)]
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    #[inline(always)]
    pub fn normalize(&self) -> Vec3 {
        let len = self.length();
        Vec3 {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }
    #[inline(always)]
    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    #[inline(always)]
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
    #[inline(always)]
    pub fn add(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
    #[inline(always)]
    pub fn mul(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
    #[inline(always)]
    pub fn scalar_mul(&self, scalar: f64) -> Vec3 {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
    #[inline(always)]
    pub fn sub(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
    #[inline(always)]
    pub fn rotate(self, euler_angles: &Vec3) -> Vec3 {
        let (sx, cx) = euler_angles.x.sin_cos();
        let (sy, cy) = euler_angles.y.sin_cos();
        let (sz, cz) = euler_angles.z.sin_cos();

        let v = Vec3::new(
            self.x,
            self.y * cx - self.z * sx,
            self.y * sx + self.z * cx,
        );

        let v = Vec3::new(
            v.x * cy + v.z * sy,
            v.y,
            -v.x * sy + v.z * cy,
        );

        Vec3::new(
            v.x * cz - v.y * sz,
            v.x * sz + v.y * cz,
            v.z,
        )
    }

    pub const ZERO: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
}

#[derive(Copy,Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }
}