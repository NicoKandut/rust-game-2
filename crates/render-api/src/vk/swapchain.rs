use ash::vk;
use crate::vk::VkRenderApi;

impl VkRenderApi {
    pub fn create_swapchain(&self, surface: vk::SurfaceKHR, width: usize, height: usize) -> (ash::khr::swapchain::Device, vk::SwapchainKHR) {
        let (format, color_space) = self.get_surface_format(surface);
        println!("FORMAT: {:?}", format);
        println!("COLOR SPACE: {:?}", color_space);
        let present_mode = self.get_present_mode(surface);
        let extent = vk::Extent2D { width: width as u32, height: height as u32 };
        let swapchain_create_info = vk::SwapchainCreateInfoKHR::default()
            .surface(surface)
            .min_image_count(2)
            .image_format(format)
            .image_color_space(color_space)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .pre_transform(vk::SurfaceTransformFlagsKHR::IDENTITY)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true);

        unsafe {
            let swapchain_extension = ash::khr::swapchain::Device::new(&self.instance.clone(), &self.device.clone());
            let swapchain = swapchain_extension
                .create_swapchain(&swapchain_create_info, None)
                .expect("Failed to create swapchain!");

            (swapchain_extension, swapchain)
        }
    }

    pub fn destroy_swapchain(&self, swapchain: vk::SwapchainKHR) {
        unsafe {
            self.swapchain_extension.destroy_swapchain(swapchain, None)
        }
    }
}