use std::hash::Hash;
use std::sync::Arc;

use crate::bvh::BVHNode;
use crate::camera::Camera;
use crate::geometry::Hittable;
use crate::material::Dielectric;
use crate::material::Lambertian;
use crate::material::Material;
use crate::material::Metal;
use crate::vec3::Vec3;
use fastrand;
use rayon::prelude::*;

#[cfg(feature = "native")]
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};

pub struct World {
    // one world has one camera
    camera: Camera,
    img_buffer: Vec<u8>,
    objects: BVHNode,
    termination_prob: f64,
    samples: usize,
}

impl World {
    pub fn new(
        camera: Camera,
        objects: BVHNode,
        samples: Option<usize>,
        termination_prob: Option<f64>,
    ) -> Self {
        let img_buffer = vec![0; camera.width_px * camera.height_px * 3];
        World {
            camera,
            img_buffer,
            objects,
            termination_prob: termination_prob.unwrap_or(0.01),
            samples: samples.unwrap_or(20),
        }
    }
    pub fn new_random_spheres(camera: Camera, num_spheres: usize) -> Self {
        let mut objects_vec: Vec<Arc<dyn Hittable>> = Vec::new();
        for _ in 0..num_spheres {
            let radius = fastrand::f64() * 0.5 + 0.1;
            let center = Vec3::new(
                fastrand::f64() * 20.0 - 10.0,
                -1.0 + radius,
                fastrand::f64() * -20.0 - 5.0,
            );

            let rand_type = fastrand::u8(0..3 as u8);
            let mat: Box<dyn Material>;
            match rand_type {
                0 => {
                    mat = Box::new(Lambertian {
                        albedo: Vec3::new(fastrand::f64(), fastrand::f64(), fastrand::f64()),
                    });
                }
                1 => {
                    mat = Box::new(Metal {
                        albedo: Vec3::new(fastrand::f64(), fastrand::f64(), fastrand::f64()),
                        fuzz: fastrand::f64() * 0.5,
                    });
                }
                2 => {
                    mat = Box::new(Dielectric {
                        albedo: Vec3::new(1.0, 1.0, 1.0),
                        refractive_index: fastrand::f64() * 2.0 + 1.0,
                    });
                }
                _ => unreachable!(),
            }
            objects_vec.push(Arc::new(crate::geometry::sphere::Sphere {
                center: center,
                radius: radius,
                material: mat,
            }));
        }

        let ground_material = Box::new(Lambertian {
            albedo: Vec3::new(0.5, 0.5, 0.5),
        });
        objects_vec.push(Arc::new(crate::geometry::sphere::Sphere {
            center: Vec3::new(0.0, -1001.0, -5.0),
            radius: 1000.0,
            material: ground_material,
        }));
        let objects = BVHNode::of_objects_and_endpoints(&mut objects_vec);

        return World::new(camera, objects, None, None);
    }

    pub fn render(&mut self) {
        let width = self.camera.width_px;
        let height = self.camera.height_px;
        let total = width * height;

        #[cfg(feature = "native")]
        let pixels: Vec<[u8; 3]> = {
            let pb = ProgressBar::new(height as u64);
            pb.set_style(
                ProgressStyle::with_template("{wide_bar} {pos}/{len} ({eta}) | ({elapsed} elapsed)")
                    .expect("invalid progress bar template")
                    .progress_chars("=>-"),
            );
            let mut out = vec![[0u8; 3]; total];
            out.par_chunks_mut(width)
                .progress_with(pb.clone())
                .enumerate()
                .for_each(|(y, row)| {
                    for x in 0..width {
                        row[x] = self.cast_rays_and_average(x, y, self.samples);
                    }
                });
            pb.finish_and_clear();
            out
        };

        #[cfg(not(feature = "native"))]
        let pixels: Vec<[u8; 3]> = {
            let mut out = vec![[0u8; 3]; total];
            out.par_chunks_mut(width).enumerate().for_each(|(y, row)| {
                for x in 0..width {
                    row[x] = self.cast_rays_and_average(x, y, self.samples);
                }
            });
            out
        };

        for (i, pixel) in pixels.iter().enumerate() {
            let x = i % self.camera.width_px;
            let y = i / self.camera.width_px;
            self.write_pixel(x, y, *pixel);
        }
    }

    pub fn render_single_threaded(&mut self) {
        for y in 0..self.camera.height_px {
            for x in 0..self.camera.width_px {
                let color = self.cast_rays_and_average(x, y, self.samples);
                self.write_pixel(x, y, color);
            }
        }
    }

    pub fn cast_rays_and_average(&self, x: usize, y: usize, samples: usize) -> [u8; 3] {
        let mut color_accumulator = Vec3::new(0.0, 0.0, 0.0);
        for _ in 0..samples {
            color_accumulator = color_accumulator.add(&self.cast_ray(x, y));
        }
        [
            ((color_accumulator.x / samples as f64).sqrt() * 255.0).clamp(0.0, 255.0) as u8,
            ((color_accumulator.y / samples as f64).sqrt() * 255.0).clamp(0.0, 255.0) as u8,
            ((color_accumulator.z / samples as f64).sqrt() * 255.0).clamp(0.0, 255.0) as u8,
        ]
    }

    pub fn cast_ray(&self, x: usize, y: usize) -> Vec3 {
        let mut current_ray = self.camera.get_ray_direction(x, y);
        let mut current_color = Vec3::new(1.0, 1.0, 1.0);
        let mut depth: u32 = 0;
        let max_depth: u32 = 100;
        loop {
            if depth >= max_depth {
                break;
            }
            if let Some(hit) = self.objects.hit(&current_ray, f64::INFINITY) {
                let emitted = hit.material.emitted(&current_ray, &hit);
                if let Some((scattered, attenuation)) = hit.material.scatter(&current_ray, &hit) {
                    current_ray = scattered;
                    current_color = current_color.mul(&attenuation);
                    current_color = current_color.add(&emitted);
                    if current_color.max_component() < 0.01 {
                        return Vec3::ZERO;
                    }
                } else {
                    return current_color.mul(&emitted);
                }
            } else {
                let unit_dir = current_ray.direction.normalize();
                let t = 0.5 * (unit_dir.y + 1.0);
                let sky_color_top = Vec3::new(9.0 / 255.0, 19.0 / 255.0, 84.0 / 255.0);
                let sky_color_bottom = Vec3::new(27.0 / 255.0, 11.0 / 255.0, 150.0 / 255.0);
                let sky = sky_color_bottom
                    .scalar_mul(1.0 - t)
                    .add(&sky_color_top.scalar_mul(t));
                return current_color.mul(&sky);
            }
            depth += 1;

            // Throughput-based Russian roulette (after a small minimum depth).
            // `termination_prob` is used as a *minimum survival probability* clamp.
            if depth >= 5 {
                let p = current_color
                    .max_component()
                    .clamp(self.termination_prob, 0.95)
                    .max(1e-12);
                if fastrand::f64() > p {
                    break;
                }
                current_color = current_color.scalar_mul(1.0 / p);
            }
        }
        Vec3::ZERO
    }
    fn write_pixel(&mut self, x: usize, y: usize, color: [u8; 3]) {
        let index = (y * self.camera.width_px + x) * 3;
        self.img_buffer[index] = color[0];
        self.img_buffer[index + 1] = color[1];
        self.img_buffer[index + 2] = color[2];
    }

    pub fn hash_buf(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.img_buffer.hash(&mut hasher);
        hasher.finish()
    }

    pub fn take_buffer_rgba(&mut self) -> Vec<u8> {
        let mut rgba = Vec::with_capacity(self.camera.width_px * self.camera.height_px * 4);
        for chunk in self.img_buffer.chunks(3) {
            rgba.push(chunk[0]);
            rgba.push(chunk[1]);
            rgba.push(chunk[2]);
            rgba.push(255);
        }
        rgba
    }

    #[cfg(feature = "native")]
    pub fn save_image(&self, filename: &str) {
        let img = image::RgbImage::from_raw(
            self.camera.width_px as u32,
            self.camera.height_px as u32,
            self.img_buffer.clone(),
        )
        .expect("invalid image buffer size");

        img.save(filename).expect("failed to save PNG image");
    }
}
