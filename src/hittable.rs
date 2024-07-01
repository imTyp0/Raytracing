use crate::{vec3::Vec3, ray::Ray};
pub struct HittableList{
	pub list: Vec<Box<dyn Hittable>>
}

impl Hittable for HittableList{
	fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
		let mut temp_record = HitRecord::default();
		let mut has_hit = false;
		let mut closest = ray_tmax;

		for object in &self.list{
			if object.hit(r, ray_tmin, closest, &mut temp_record){
				has_hit = true;
				closest = temp_record.t;
				*rec = temp_record;
			}
		}

		has_hit
	}
}

#[derive(Clone, Copy)]
pub struct HitRecord{
	pub p: Vec3,
	pub normal: Vec3,
	pub t: f64,
	pub front_face: bool
}

impl Default for HitRecord{
	fn default() -> Self {
		HitRecord{
			p: Vec3::new(0., 0., 0.),
			normal: Vec3::new(0., 0., 0.),
			t: 0.,
			front_face: true
		}
	}
}

impl HitRecord{
	// Make sure the normal vector is always pointing out from the surface
	// even if the ray comes from within the mesh
	// Note: param `outward_normal` is expected to be normalized
	pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3){
		self.front_face = r.direction.dot(&outward_normal) < 0.;
		self.normal = if self.front_face {outward_normal} else {-outward_normal};
	}
}

pub trait Hittable{
	fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool;
}