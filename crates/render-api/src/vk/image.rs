use ash::vk;
use crate::vk::VkRenderApi;

impl VkRenderApi {
    pub fn create_image(&self, width: u32, height: u32, usage: vk::ImageUsageFlags) -> (vk::Image, vk::DeviceMemory) {
        let create_info = vk::ImageCreateInfo::default()
            .image_type(vk::ImageType::TYPE_2D)
            .format(vk::Format::B8G8R8A8_SRGB)
            .extent(vk::Extent3D { width, height, depth: 1 })
            .mip_levels(1)
            .array_layers(1)
            .samples(vk::SampleCountFlags::TYPE_1)
            .tiling(vk::ImageTiling::OPTIMAL)
            .usage(usage)
            .sharing_mode(vk::SharingMode::EXCLUSIVE)
            .initial_layout(vk::ImageLayout::UNDEFINED);
        let image = unsafe { self.device.create_image(&create_info, None).unwrap() };
        let memory = self.allocate_memory_image(
            image,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        );

        unsafe {
            self.device.bind_image_memory(image, memory, 0)
                .expect("Failed to bind image memory!");
        }

        (image, memory)
    }

    pub fn create_texture_image(&self, width: u32, height: u32) -> (vk::Image, vk::DeviceMemory) {
        self.create_image(width, height, vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED)
    }

    pub fn destroy_image(&self, image: vk::Image) {
        unsafe {
            self.device.destroy_image(image, None);
        }
    }
}