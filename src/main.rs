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

const ASPECT_RATIO: f64 = 16. / 9.;
const WIDTH: u16 = 400;

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
	const HEIGHT: u16 = (WIDTH as f64 / ASPECT_RATIO) as u16;
	
	// Adding objects to the world
	let mut world = HittableList{
		list: vec![]
	};
	// world.list.push(Box::new(Plane::new(
	// 	Vec3::new(0., -1., 0.),
	// 	Vec3::new(-1., 1., 0.).normalize()
	// )));
	world.list.push(Box::new(Plane::new(
		Vec3::new(0., -1., 0.),
		Vec3::new(0., 1., 0.)
	)));
	world.list.push(Box::new(Sphere::new(
		Vec3::new(-2., 1., -5.), 1.
	)));
	world.list.push(Box::new(Sphere::new(
		Vec3::new(2., 1., -3.), 1.
	)));

	// Calculate constants relative to the camera
	const FOCAL_LENGTH: f64 = 1.;
	const VIEWPORT_HEIGHT: f64 = 2.0;
	const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * ASPECT_RATIO;
	const CAMERA_CENTER: Vec3 = Vec3::zero();

	// Calculate the vectors across the horizontal and down the vertical viewport edges
	const VIEWPORT_U: Vec3 = Vec3::new(VIEWPORT_WIDTH, 0., 0.);
	const VIEWPORT_V: Vec3 = Vec3::new(0., -VIEWPORT_HEIGHT, 0.);

	// Calculate the horizontal and vertical delta vectors from pixel to pixel
	let pixel_delta_u = VIEWPORT_U / WIDTH as f64;
	let pixel_delta_v = VIEWPORT_V / HEIGHT as f64;

	// Calculate the location of the upper left pixel
	let viewport_upper_left: Vec3 = 
		CAMERA_CENTER + -Vec3::new(0., 0., FOCAL_LENGTH) + -VIEWPORT_U/2. + -VIEWPORT_V/2.;
	let pixel00_loc: Vec3 = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

	/* 
		Rendering
	*/

	// Output file pointer
	let mut filp = fs::File::create("image.ppm").expect("Error creating the file.");

	// Image header
	let header = format!("P3\n{WIDTH} {HEIGHT}\n255\n");
	filp.write(header.as_bytes()).expect("Error writing header.");

	// Image body
	for row in 0..HEIGHT{
		// Print progress
		print!("\rScanlines remaining: {} ", HEIGHT-row);
		io::stdout().flush().unwrap();

		for col in 0..WIDTH{
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