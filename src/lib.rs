pub mod objects;
pub mod lights;

use std::rc::Rc;

use crate::objects::{Intersectable, RayHit};
use crate::lights::*;

use glam::{Vec2, Vec3};

const SKY_COLOR: u32 = rgb_u8(70, 180, 245);

pub struct Ray {
    position: Vec3,
    direction: Vec3,
}

pub struct RayPayload {
    hit_obj: Rc<dyn Intersectable>,
    ray_hit: RayHit,
}

pub struct Camera {
    pub position: Vec3,
    pub forward: Vec3,
    pub up: Vec3,
}

impl Camera {
    pub fn new(position: Vec3, forward: Vec3) -> Self {
        Self {
            position,
            forward: forward.normalize(),
            up: Vec3::Y,
        }
    }

    pub fn ray(&self, uv: Vec2) -> Ray {
        let target = self.position + self.forward - uv.y * self.up + uv.x * self.up.cross(self.forward);
        Ray {
            position: self.position,
            direction: target - self.position,
        }
    }
}

#[derive(Default)]
pub struct Scene {
    // Move to arena allocator eventually? https://docs.rs/typed-arena/2.0.1/typed_arena/
    // Also may have to use Rc instead of Box
    pub objects: Vec<Rc<dyn Intersectable>>,
    pub lights: Vec<Rc<dyn LightEmitting>>,
}

// uv (0, 0) is top left
pub fn render(camera: &Camera, scene: &Scene, size: (usize, usize)) -> Vec<u32> {
    let (size_x, size_y) = size;
    let mut buffer: Vec<u32> = vec![0; size_x * size_y];
    for y in 0..size_y {
        for x in 0..size_x {
            let index = (y * size_x + x) as usize;
            let mut uv = Vec2 {
                x: (x as f32 / size_x as f32),
                y: y as f32 / size_y as f32,
            };
            uv -= Vec2::splat(0.5); // Adjust from [0, 1] to [-0.5, 0.5]
            uv.x *= size_x as f32 / size_y as f32; // Adjust for aspect ratio of the window
            buffer[index] = pixel(camera, scene, uv);
        }
    }
    buffer
}

fn pixel(camera: &Camera, scene: &Scene, uv: Vec2) -> u32 {
    if let Some(ray_result) = raycast(&camera.ray(uv), scene) {
        // rgb_vec(ray_result.ray_hit.surface_normal)
        rgb_vec(light(&ray_result.ray_hit, scene))
    } else {
        SKY_COLOR
    }
}

fn raycast(ray: &Ray, scene: &Scene) -> Option<RayPayload> {
    let mut closest_hit: Option<RayPayload> = None;
    let min_t: f32 = f32::MAX;

    for object in scene.objects.iter() {
        if let Some(hit) = object.intersects(ray) {
            if hit.t < min_t {
                closest_hit = Some(RayPayload { hit_obj: object.clone(), ray_hit: hit });
            }
        }
    }

    closest_hit
}

fn light(rayhit: &RayHit, scene: &Scene) -> Vec3 {
    
    let mut cumulative_light: Vec3 = Vec3::ZERO;

    for light in scene.lights.iter() {
        // Check LOS first, see if we hit anything on the way to the light
        let los_ray = Ray {
            position: rayhit.position + rayhit.surface_normal * 0.001,
            direction: light.dir_from(&rayhit.position)
        };

        if let Some(los_rayhit) = raycast(&los_ray, scene) {
            // If we hit something, then we are in shadow for this light source
            continue;
        } else {
            cumulative_light += light.light_on(&rayhit.position, &rayhit.surface_normal); 
        }
    }

    cumulative_light
}

fn rgb_f32(r: f32, g: f32, b: f32) -> u32 {
    let r_u8 = (255.0 * r) as u32;
    let g_u8 = (255.0 * g) as u32;
    let b_u8 = (255.0 * b) as u32;
    r_u8 << 16 | g_u8 << 8 | b_u8
}

fn rgb_vec(rgb: Vec3) -> u32 {
    rgb_f32(rgb.x, rgb.y, rgb.z)
}

const fn rgb_u8(r: u8, g: u8, b: u8) -> u32 {
    (r as u32) << 16 | (g as u32) << 8 | (b as u32)
}