use ash::vk::{Queue, SurfaceKHR};
use glfw::PWindow;

use render_api::vk::VkRenderApi;

pub struct Renderer {
    api: VkRenderApi,
    surface: SurfaceKHR,
    queue: Queue,
}

impl Renderer {
    pub fn new(p_window: &PWindow) -> Self {
        let api = VkRenderApi::default();

        let surface = api.create_surface(p_window);
        let queue = api.create_queue();

        Self {
            api,
            surface,
            queue,
        }
    }

    pub fn render(&self) {
        println!("Rendering...");
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {}
    }
}