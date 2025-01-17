pub use crate::vec3::Vec3 as Color;
use std::{fs::File, io::Write};

// Blend between two colors based on a parameter
pub fn lerp_colors(a: f64, start: Color, end: Color) -> Color{
	start * (1. - a) + end * a
}

// Write a pixel's color to a file pointer (ppm format)
pub fn write_color(filp: &mut File, pixel_color: Color){
	// Taking the sqrt() means going from linear color space to gamma corrected space
    let r = pixel_color.x.sqrt();
	let g = pixel_color.y.sqrt();
	let b = pixel_color.z.sqrt();

	// Take RGB [0,1] to RGB [0, 255]
	let rbyte = (256. * r.clamp(0., 0.999)) as u8;
	let gbyte = (256. * g.clamp(0., 0.999)) as u8;
	let bbyte = (256. * b.clamp(0., 0.999)) as u8;

	// Write the RGB values of the pixel to the file
	let line = format!("{rbyte} {gbyte} {bbyte}\n");
	filp.write_all(line.as_bytes()).expect("Error writing a line.");
}
