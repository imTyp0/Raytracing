mod camera;
use camera::Camera;
mod color;
mod geometry;
use geometry::*;
mod hittable;
use hittable::HittableList;
mod ray;
mod vec3;
use vec3::Vec3;

use std::time::Instant;

fn main() {
	/* 
		Setting up the scene
	*/
	// Adding objects to the world

    let mut world = HittableList{
		list: vec![]
	};
	world.list.push(Box::new(Plane::new(
		Vec3::new(0., -1., 0.),
    	Vec3::new(0., 1., 0.)
    )));
	world.list.push(Box::new(Sphere::new(
		Vec3::new(0., 1., -2.), 1.5
	)));
	world.list.push(Box::new(Sphere::new(
		Vec3::new(3.5, 0., -2.), 1.
	)));
	world.list.push(Box::new(Sphere::new(
		Vec3::new(-2.5, 0., -2.), 1.
	)));

	// Creating a camera and initializing it
	let mut cam = Camera{
		aspect_ratio: 16. /9.,
		image_width: 800,
        samples_per_pixel: 16,
        max_bounces: 6,
		center: Some(Vec3::new(0., 0., 15.)),
		..Default::default()
	};
    
    // Benchmarking
    let before_render = Instant::now();

	// Rendering
	cam.render(&world);

    println!("Took {} seconds to render.", before_render.elapsed().as_secs());
}
