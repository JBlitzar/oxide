mod aabb;
mod bluenoise;
mod bvh;
mod camera;
mod geometry;
mod light;
mod material;
mod renderer;
mod scene;
mod sky;
mod vec3;
mod world;

use crate::camera::Camera;
use crate::geometry::mesh::MeshBVH;
use crate::scene::{HdrSkyId, MaterialDesc, ObjectDesc, SceneDescription, SkyDesc};
use crate::vec3::Vec3;

fn main() {
    fastrand::seed(42);
    let profile = env!("OXIDE_PROFILE");
    let (width, height, samples, roulette) = match profile {
        "iteration" => (960, 540, 100, 0.1),
        "extra" => (3840, 2160, 1_000, 0.05),
        _ => (1920, 1080, 100, 0.1),
    };
    println!(
        "Rendering at {}x{} with {} samples per pixel and termination probability of {}",
        width, height, samples, roulette
    );

    let scene = SceneDescription {
        camera: Camera::look_at(
            width,
            height,
            90.0_f64.to_radians(),
            Vec3::new(3.0, 2.5, 0.0),
            Vec3::new(0.5, 0.0, -5.0),
            5.385,
            0.04,
        ),
        objects: vec![
            {
                let (vertices, faces) = MeshBVH::load_stl_indexed(
                    "teapot_fixed.stl",
                    Some(2.0),
                    Some(Vec3::new(-2.0, 0.0, -5.0)),
                    None,
                );
                ObjectDesc::Mesh {
                    vertices,
                    faces,
                    material: MaterialDesc::Dielectric {
                        albedo: Vec3::new(1.0, 1.0, 1.0),
                        refractive_index: 1.7,
                    },
                }
            },
            {
                let (vertices, faces) = MeshBVH::load_stl_indexed(
                    "dragon_fixed.stl",
                    Some(2.0),
                    Some(Vec3::new(2.0, -0.5, -5.0)),
                    Some(Vec3::new(0.0, 0.0, 0.0)),
                );
                ObjectDesc::Mesh {
                    vertices,
                    faces,
                    material: MaterialDesc::Lambertian {
                        albedo: Vec3::new(0.7, 1.0, 1.0),
                    },
                }
            },
            ObjectDesc::Sphere {
                center: Vec3::new(0.0, 0.7, -5.0),
                radius: 0.7,
                material: MaterialDesc::Lambertian {
                    albedo: Vec3::new(0.2, 0.5, 0.5),
                },
            },
            ObjectDesc::Sphere {
                center: Vec3::new(0.0, -1000.0, 0.0),
                radius: 1000.0,
                material: MaterialDesc::Checkerboard {
                    color_a: Vec3::new(0.0, 0.0, 0.0),
                    color_b: Vec3::new(1.0, 1.0, 1.0),
                    scale: 1.0,
                },
            },
        ],
        sky: SkyDesc::Hdr {
            id: HdrSkyId::CitrusOrchard,
            exposure: 0.3,
        },
        samples,
        termination_prob: roulette,
    };

    // let scene = SceneDescription::load("demo.scene");
    let (world, mut renderer) = scene.build();
    let start = std::time::Instant::now();
    renderer.render(&world);
    println!("Render time: {:?}", start.elapsed());
    renderer.save_image("output.png");
    println!("Image hash: {:x}", renderer.hash_buf());
    scene.save("demo.scene")
}

#[cfg(test)]
mod tests {
    use super::*;
}
