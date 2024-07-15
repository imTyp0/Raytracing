use std::ops::*;
use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct Vec3{
	pub x: f64,
	pub y: f64,
	pub z: f64
}

// Base implementations
impl Vec3{
	pub const fn zero() -> Vec3{
		Vec3{x: 0., y: 0., z: 0.}
	}
	pub const fn new(x: f64, y: f64, z: f64) -> Vec3{
		Vec3{x,y,z}
	}
	pub fn len_squared(&self) -> f64{
		self.x.powf(2.)+self.y.powf(2.)+self.z.powf(2.)
	}
	pub fn len(&self) -> f64{
		self.len_squared().powf(0.5)
	}
	pub fn dot(&self, b: &Vec3) -> f64{
		self.x*b.x + self.y*b.y + self.z*b.z
	}
	pub fn _cross(&self, b: &Vec3) -> Vec3{
		Vec3::new(
			self.y * b.z - self.z * b.y,
			self.z * b.x - self.x * b.z,
			self.x * b.y - self.y * b.x
		)
	}
	pub fn normalize(&self) -> Vec3{
		let len = Vec3::len(self);
		Vec3::new(
			self.x / len,
			self.y / len,
			self.z / len
		)
	}
    pub fn random_range(min: f64, max: f64) -> Vec3{
        let mut rng = rand::thread_rng();
        Vec3::new(
            rng.gen_range(min..max), rng.gen_range(min..max), rng.gen_range(min..max)
        )
    }
    pub fn _random_on_hemisphere(normal: &Vec3) -> Vec3{
        loop{
            let v = Vec3::random_range(-1., 1.);
            if v.len_squared() <= 1.{
                // Dot product > 0 if pointing in the same direction
                if v.normalize().dot(normal) > 0.{
                    return v;
                }
                else{
                    return -v;
                }
            }
        }
    }
}

// Operators
impl Neg for Vec3{
	type Output = Vec3;
	fn neg(self) -> Vec3{
		Vec3::new(-self.x, -self.y, -self.z)
	}
}
impl Add for Vec3{
	type Output = Vec3;
	fn add(self, rhs: Self) -> Vec3{
		Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
	}
}
impl Add<f64> for Vec3{
	type Output = Vec3;
	fn add(self, rhs: f64) -> Vec3{
		Vec3::new(self.x + rhs, self.y + rhs, self.z + rhs)
	}
}
impl AddAssign for Vec3{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl Mul<f64> for Vec3{
	type Output = Vec3;
	fn mul(self, rhs: f64) -> Vec3{
		Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
	}
}
impl Div<f64> for Vec3{
	type Output = Vec3;
	fn div(self, rhs: f64) -> Vec3{
		Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
	}
}
