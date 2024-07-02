pub use crate::vec3::Vec3 as Color;
use std::{fs::File, io::Write};

// Blend between two colors based on a parameter
pub fn lerp_colors(a: f64, start: Color, end: Color) -> Color{
	start * (1. - a) + end * a
}

// Write a pixel's color to a file pointer (ppm format)
pub fn write_color(filp: &mut File, pixel_color: &Color){
	let r = pixel_color.x;
	let g = pixel_color.y;
	let b = pixel_color.z;

	// Take RGB [0,1] to RGB [0, 255]
	let rbyte = (255.999 * r) as u8;
	let gbyte = (255.999 * g) as u8;
	let bbyte = (255.999 * b) as u8;

	// Write the RGB values of the pixel to the file
	let line = format!("{rbyte} {gbyte} {bbyte}\n");
	filp.write(line.as_bytes()).expect("Error writing a line.");
}