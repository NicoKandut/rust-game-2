use crate::vk::VkRenderApi;
use ash::vk;
use std::ptr::null;

impl VkRenderApi {
    pub fn create_surface(&self, window: &glfw::Window) -> vk::SurfaceKHR {
        let mut surface = vk::SurfaceKHR::null();
        window
            .create_window_surface(self.instance.handle(), null(), &mut surface)
            .result()
            .expect("Failed to create window surface!");

        let (format, color_space) = self.get_surface_format(surface);
        println!("FORMAT: {:?}", format);
        println!("COLOR SPACE: {:?}", color_space);

        surface
    }

    pub fn destroy_surface(&self, surface: vk::SurfaceKHR) {
        unsafe {
            self.surface_extension.destroy_surface(surface, None);
        }
    }
}