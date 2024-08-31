pub mod mat2;
pub mod mat3;
pub mod mat4;
pub mod vec2;
pub mod vec3;
pub mod vec4;
pub mod vec2mat2;
pub mod vec3mat3;
pub mod vec4mat4;

pub trait Dot {
    fn dot(self, rhs: Self) -> f32;
}

pub trait Cross {
    type Output;

    fn cross(self, rhs: Self) -> Self::Output;
}

pub trait Norm {
    fn norm(&self) -> f32 {
        self.norm2().sqrt()
    }

    fn norm2(&self) -> f32;
}

pub trait Identity {
    fn identity() -> Self;
}

pub trait Normalize {
    type Output;

    fn normalize(self) -> Self::Output;
}