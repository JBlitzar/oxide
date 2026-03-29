use crate::vec3::{Ray, Vec3};

pub struct Camera {
    pub(crate) width_px: usize,
    pub(crate) height_px: usize,
    position: Vec3,
    forward: Vec3,
    right: Vec3,
    up: Vec3,
    half_tan_fov_x: f64,
    half_tan_fov_y: f64,
    focus_distance: f64,
    aperture: f64,
}
impl Camera {
    pub fn look_at(
        width_px: usize,
        height_px: usize,
        x_fov: f64,
        position: Vec3,
        target: Vec3,
        focus_distance: f64,
        aperture: f64,
    ) -> Self {
        let world_up = Vec3::new(0.0, 1.0, 0.0);
        let forward = target.sub(&position).normalize();
        let right = forward.cross(&world_up).normalize();
        let up = right.cross(&forward).normalize();

        let half_tan_fov_x = (x_fov / 2.0).tan();
        let half_tan_fov_y = half_tan_fov_x * (height_px as f64 / width_px as f64);
        Camera {
            width_px,
            height_px,
            position,
            forward,
            right,
            up,
            half_tan_fov_x,
            half_tan_fov_y,
            focus_distance,
            aperture,
        }
    }

    pub fn get_ray_direction(&self, x: usize, y: usize) -> Ray {
        let x_cmp =
            ((x as f64 + fastrand::f64()) / self.width_px as f64 - 0.5) * self.half_tan_fov_x;
        let y_cmp =
            (0.5 - (y as f64 + fastrand::f64()) / self.height_px as f64) * self.half_tan_fov_y;
        let dir = self
            .forward
            .add(&self.right.scalar_mul(x_cmp))
            .add(&self.up.scalar_mul(y_cmp))
            .normalize();

        let focus_point = self.position.add(&dir.scalar_mul(self.focus_distance));
        let lens_offset = random_in_unit_disk().scalar_mul(self.aperture / 2.0);
        let origin = self
            .position
            .add(&self.right.scalar_mul(lens_offset.x))
            .add(&self.up.scalar_mul(lens_offset.y));
        let new_dir = focus_point.sub(&origin).normalize();
        Ray::new(origin, new_dir)
    }
}

fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(
            fastrand::f64() * 2.0 - 1.0,
            fastrand::f64() * 2.0 - 1.0,
            0.0,
        );
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}
