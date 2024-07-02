use crate::color::*;
use crate::hittable::*;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::{fs, io::{self, Write}};

#[derive(Default)]
pub struct Camera{
	pub aspect_ratio: f64,
	pub image_width: u32,
	pub image_height: Option<u32>,
	pub center: Option<Vec3>,
	pub pixel00_loc: Option<Vec3>,
	pub pixel_delta_u: Option<Vec3>,
	pub pixel_delta_v: Option<Vec3>
}

impl Camera{
	// Render a list of hittable objects
	pub fn render(&mut self, world: &HittableList){
		self.initialize();
		let h = self.image_height.unwrap();
		let w = self.image_width;

		// Output file pointer
		let mut filp = fs::File::create("image.ppm").expect("Error creating the file.");

		// Image header
		let header = format!("P3\n{} {}\n255\n", w, h);
		filp.write(header.as_bytes()).expect("Error writing header.");

		// Image body
		for row in 0..h{
			// Print progress
			print!("\rLines remaining: {} ", h-row);
			io::stdout().flush().unwrap();

			for col in 0..w{
				let pixel_center =
					self.pixel00_loc.unwrap() + self.pixel_delta_u.unwrap() * (col as f64) + self.pixel_delta_v.unwrap() * (row as f64);
				let ray_direction = pixel_center + -self.center.unwrap();

				let r = Ray::new(self.center.unwrap(), ray_direction);
				let pixel_color = Camera::ray_color(&r, &world);

				write_color(&mut filp, &pixel_color);
			}
		}

		println!("\rDone.                 ");
	}

	// Initialize the camera values or leave them as is
	fn initialize(&mut self){
		self.image_height = Some((self.image_width as f64 / self.aspect_ratio) as u32);
		if self.center.is_none(){
			self.center = Some(Vec3::zero());
		}

		// Viewport dimensions
		let focal_point = Vec3::new(0., 0., -10.);
		let viewport_height = 4_f64;
		let viewport_width = viewport_height * (self.image_width as f64 / self.image_height.unwrap() as f64);

		// Calculate the vectors across the horizontal and down the vertical viewport edges
		let viewport_u = Vec3::new(viewport_width, 0., 0.);
		let viewport_v = Vec3::new(0., -viewport_height, 0.);

		// Calculate the horizontal and vertical delta vectors from pixel to pixel
		self.pixel_delta_u = Some(viewport_u / self.image_width as f64);
		self.pixel_delta_v = Some(viewport_v / self.image_height.unwrap() as f64);

		// Calculate the location of the upper left pixel
		self.pixel00_loc = Some(
			Vec3::new(-viewport_width * 0.5, viewport_height * 0.5, 0.) + focal_point + self.center.unwrap()
		);
		
	}

	// Returns the color of a ray, based on what it hit
	fn ray_color(r: &Ray, world: &HittableList) -> Color{
		let mut hit_record = HitRecord::default();
	
		if world.hit(r, 0., f64::INFINITY, &mut hit_record){
			return (hit_record.normal + Color::new(1., 1., 1.)) * 0.5;
		}
	
		let normalized_r = r.direction.normalize();
		let a = (normalized_r.y + 1.0) * 0.5;
		lerp_colors(a, Color::new(1., 1., 1.), Color::new(0.5, 0.7, 1.))
	}
}