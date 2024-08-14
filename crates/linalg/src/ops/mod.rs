pub mod vec2mat2;
pub mod vec3mat3;
pub mod mat2;
pub mod mat3;
pub mod vec2;
pub mod vec3;

pub trait Dot {
    fn dot(self, rhs: Self) -> f64;
}

pub trait Cross {
    fn cross(self, rhs: Self) -> Self;
}

pub trait Norm {
    fn norm(&self) -> f64 {
        self.norm2().sqrt()
    }

    fn norm2(&self) -> f64;
}