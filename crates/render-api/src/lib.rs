pub mod vk;
mod wgpu;

pub trait RenderApi {
    fn new() -> Self;
}