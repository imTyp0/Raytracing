use crate::{
	vec3::Vec3,
	ray::Ray,
	hittable::*
};
pub struct Sphere{
	pub center: Vec3,
	pub radius: f64
}

impl Sphere{
	pub fn new(center: Vec3, radius: f64) -> Sphere{
		Sphere{center, radius}
	}
}
impl Hittable for Sphere{
	fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
		let center_shifted = self.center + -r.origin;
		// Quadratic formula for wether or not the ray hit
		// (x-center_x)^2 + (y-center_y)^2 + (z-center_z)^2 <= r^2
		let a = r.direction.len_squared();
		let h = r.direction.dot(&center_shifted);
		let c = center_shifted.len_squared() - self.radius * self.radius;
		let delta = h*h - a * c;
	
		// delta < 0 means no intersections -> return false
		// delta >= 0 means the ray intersected at one or more
		// points -> return the closest root
		if delta < 0.{
			return false;
		}

		let sqrt_delta = delta.sqrt();
		// Find the nearest root that lies in the acceptable range.
		let mut root = (h - sqrt_delta) / a;
		if root <= ray_tmin || root >= ray_tmax{
			root = (h + sqrt_delta) / a;
			if root <= ray_tmin || root >= ray_tmax{
				return false;
			}
		}

		rec.t = root;
        rec.point = r.at(rec.t);
        let outward_normal = (rec.point + -self.center) / self.radius;
		rec.set_face_normal(r, outward_normal);

		true
	}
}

pub struct Plane{
	point: Vec3,	// A point in the plane
	normal: Vec3,	// The plane's normal vector, normalized
}

impl Plane{
	pub fn new(point: Vec3, normal: Vec3) -> Plane{
		Plane{
			point,
			normal
		}
	}
}

impl Hittable for Plane{
	fn hit(&self, r: &Ray, _ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
		// t = [ n . (p - o) ] / (n . d)
		// gives us the t at which the ray intersects the plane
		// if t == t_max or t is negative, we shot the ray into infinity
		let t = (self.normal.dot(&(self.point + -r.origin))) / (self.normal.dot(&r.direction));
		if t <= 0. || t >= ray_tmax{
			return false;
		}

		rec.t = t;
		rec.point = r.at(t);
		rec.set_face_normal(r, self.normal);
		true
	}
}