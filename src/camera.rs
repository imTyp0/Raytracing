use crate::color::*;
use crate::hittable::*;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::{
    fs,
    io::{self, Write}
};
use rayon::prelude::*;

#[derive(Default)]
pub struct Camera{
	pub aspect_ratio: f64,
	pub image_width: u32,
    pub samples_per_pixel: u16,
	pub max_bounces: u8,
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
		let h = self.image_height.unwrap() as usize;
		let w = self.image_width as usize;

		// Output file pointer
		let mut filp = fs::File::create("image.ppm").expect("Error creating the file.");

		// Image header
		let header = format!("P3\n{} {}\n255\n", w, h);
		filp.write_all(header.as_bytes()).expect("Error writing header.");
        
        // Buffer for the pixel colors
        let mut image = vec![vec![Color::zero(); w]; h];

        // Pixel counter
        let mut samples_done = 0;

        // Image body
        while samples_done < self.samples_per_pixel{
            // For each pixel of the image
            image.par_iter_mut().enumerate().for_each(|(y, row)|{
                row.par_iter_mut().enumerate().for_each(|(x, col)|{
                    let ray = self.get_ray(x as u32, y as u32);
                    let pixel_color = Camera::ray_color(&ray, world, self.max_bounces);
                    *col += pixel_color * 1_f64 / self.samples_per_pixel as f64;
                });
            });
            samples_done += 1;

            // Print progress
            print!("\r{}/{} samples done.", samples_done, self.samples_per_pixel);
            io::stdout().flush().unwrap();
        }
        
        // Iterate through the image and write to file pointer.
        println!("\rWriting to file...");
        for &color in image.iter().flat_map(|row| row.iter()){
            write_color(&mut filp, color);
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
	fn ray_color(r: &Ray, world: &HittableList, depth: u8) -> Color{
        // If we reached max number of bounces, return black
        if depth == 0{
            return Color::zero();
        }

        // If we hit an object in the world
        let mut hit_record = HitRecord::default();
		if world.hit(r, 0.001, f64::INFINITY, &mut hit_record){
            // let bounce_dir = Vec3::random_on_hemisphere(&hit_record.normal);
            let bounce_dir = hit_record.normal + Vec3::random_range(-1., 1.).normalize();
            return Camera::ray_color(&Ray::new(hit_record.point, bounce_dir), world, depth-1) * 0.5;
		}
        
        // If we haven't hit anything, color the sky based on the height of the vector (gradient)
		let normalized_r = r.direction.normalize();
		let a = (normalized_r.y + 1.0) * 0.5;
		lerp_colors(a, Color::new(1., 1., 1.), Color::new(0.5, 0.7, 1.))
	}
    // Makes a ray originating from the Camera, and pointed at a randomly sampled point around
    // pixel (i, j)
    fn get_ray(&self, i: u32, j: u32) -> Ray{
        // Generate a random vector in the unit square [-0.5, -0.5] [0.5, 0.5]
        let offset = Vec3::new(
            rand::random::<f64>() - 0.5, rand::random::<f64>() - 0.5, 0.
        );
        let sample = self.pixel00_loc.unwrap() +
            (self.pixel_delta_u.unwrap() * (i as f64 + offset.x)) +
            (self.pixel_delta_v.unwrap() * (j as f64 + offset.y));
        let direction = sample + -self.center.unwrap();

        Ray::new(self.center.unwrap(), direction)
    }
}
