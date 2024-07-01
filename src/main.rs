mod vec3;
use vec3::Vec3;
mod hittable;
use hittable::{HitRecord, Hittable, HittableList};
mod color;
use color::Color;
mod ray;
use ray::Ray;
mod geometry;
use geometry::*;
use std::{fs, io::{self, Write}};

const IMAGE_HEIGHT: u16 = 225;
const IMAGE_WIDTH: u16 = 400;
const ASPECT_RATIO: f64 = (IMAGE_HEIGHT as f64) / (IMAGE_WIDTH as f64);
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT / ASPECT_RATIO;

fn lerp_colors(a: f64, start: Color, end: Color) -> Color{
	start * (1. - a) + end * a
}

// Returns the color of a given ray
fn ray_color(r: &Ray, world: &HittableList) -> Color{
	let mut hit_record = HitRecord::default();

	if world.hit(r, 0., f64::INFINITY, &mut hit_record){
		return Color::new(
			hit_record.normal.x+1., hit_record.normal.y+1., hit_record.normal.z+1.
		) * 0.5;
	}

	let normalized_r = r.direction.normalize();
	let a = (normalized_r.y + 1.0) * 0.5;
	lerp_colors(a, Color::new(1., 1., 1.), Color::new(0.5, 0.7, 1.))
}

fn main() {
	/* 
		Setting up the scene
	*/
	// Calculate image height based on aspect ratio
	
	// Adding objects to the world
	let mut world = HittableList{
		list: vec![]
	};
	world.list.push(Box::new(Plane::new(
		Vec3::new(0., -1., 0.),
		Vec3::new(0., 1., 0.)
	)));
	world.list.push(Box::new(Sphere::new(
		Vec3::new(0., 0., -2.), 1.
	)));
	world.list.push(Box::new(Sphere::new(
		Vec3::new(3., 0., -2.), 1.
	)));
	world.list.push(Box::new(Sphere::new(
		Vec3::new(-3., 0., -2.), 1.
	)));

	// Calculate constants relative to the camera
	// const FOCAL_LENGTH: f64 = 1.;
	let focal_point = Vec3::new(0., 0., -0.5);
	const CAMERA_CENTER: Vec3 = Vec3::zero();

	// Calculate the vectors across the horizontal and down the vertical viewport edges
	const VIEWPORT_U: Vec3 = Vec3::new(VIEWPORT_WIDTH, 0., 0.);
	const VIEWPORT_V: Vec3 = Vec3::new(0., -VIEWPORT_HEIGHT, 0.);

	// Calculate the horizontal and vertical delta vectors from pixel to pixel
	let pixel_delta_u = VIEWPORT_U / (IMAGE_WIDTH as f64);
	let pixel_delta_v = VIEWPORT_V / (IMAGE_HEIGHT as f64);

	// Calculate the location of the upper left pixel
	let pixel00_loc: Vec3 =
	Vec3::new(-VIEWPORT_WIDTH * 0.5, VIEWPORT_HEIGHT * 0.5, 0.) + focal_point + CAMERA_CENTER;

	/* 
		Rendering
	*/

	// Output file pointer
	let mut filp = fs::File::create("image.ppm").expect("Error creating the file.");

	// Image header
	let header = format!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");
	filp.write(header.as_bytes()).expect("Error writing header.");

	// Image body
	for row in 0..IMAGE_HEIGHT{
		// Print progress
		print!("\rScanlines remaining: {} ", IMAGE_HEIGHT-row);
		io::stdout().flush().unwrap();

		for col in 0..IMAGE_WIDTH{
			let pixel_center =
				pixel00_loc + pixel_delta_u * (col as f64) + pixel_delta_v * (row as f64);
			let ray_direction = pixel_center + -CAMERA_CENTER;

			let r = Ray::new(CAMERA_CENTER, ray_direction);
			let pixel_color = ray_color(&r, &world);

			color::write_color(&mut filp, &pixel_color);
		}
	}

	println!("\rDone.                 ");
}