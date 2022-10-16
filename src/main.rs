use std::rc::Rc;

use glam::Vec3;
use minifb::{Key, Window, WindowOptions};
use rust_ray_tracer::{render, Camera, Scene, objects::*};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut window = Window::new(
        "Rust Ray Tracer - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let camera: Camera = Camera::new(Vec3::from([0., 0., 3.]), Vec3::from([0., 0., -1.]));

    let mut scene = Scene::default();
    scene.objects.push(Rc::new(Sphere::default()));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let size = (window.get_size().0, window.get_size().1);

        let frame = render(&camera, &scene, size);

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&frame, size.0, size.1).unwrap();
    }
}
