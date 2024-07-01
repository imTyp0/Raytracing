#[cfg(tests)]
mod tests{
	use super::*;

	/*
		We need tests for inputs > 0, == 0, and < 0
	 */
	// foo
	#[test]
	fn gt0(){
		let mut rec = HitRecord::default();
		let ray = &Ray::new();
		let normal = &Vec3::new(-1., 1., 0.).normalize();
		rec.set_face_normal(ray, normal);
	}
}