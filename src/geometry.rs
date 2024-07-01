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
	pub fn new(c: Vec3, r: f64) -> Sphere{
		Sphere{ center: c, radius: r}
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
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p + -self.center) / self.radius;
		rec.set_face_normal(r, outward_normal);

		true
	}
}

pub struct Plane{
	p: Vec3,	// A point in the plane
	n: Vec3,	// The plane's normal vector, normalized
}

impl Plane{
	pub fn new(p: Vec3, n: Vec3) -> Plane{
		Plane{
			p: p,
			n: n
		}
	}
}

impl Hittable for Plane{
	fn hit(&self, r: &Ray, _ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
		// t = [ n . (p - o) ] / (n . d)
		// gives us the t at which the ray intersects the plane
		// if t == t_max or t is negative, we shot the ray into infinity
		let t = (self.n.dot(&(self.p + -r.origin))) / (self.n.dot(&r.direction));
		if t <= 0. || t >= ray_tmax{
			return false;
		}

		rec.t = t;
		rec.p = r.at(t);
		rec.set_face_normal(r, self.p);
		true
	}
}