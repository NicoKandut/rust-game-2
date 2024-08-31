use ash::vk;
use crate::vk::VkRenderApi;

impl VkRenderApi {
    pub fn create_image_memory_barrier(&self, image: vk::Image, old_layout: vk::ImageLayout, new_layout: vk::ImageLayout) -> vk::ImageMemoryBarrier {
        vk::ImageMemoryBarrier::default()
            .src_access_mask(vk::AccessFlags::empty())
            .dst_access_mask(vk::AccessFlags::empty())
            .old_layout(old_layout)
            .new_layout(new_layout)
            .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .image(image)
            .subresource_range(vk::ImageSubresourceRange::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .base_mip_level(0)
                .level_count(1)
                .base_array_layer(0)
                .layer_count(1)
            )
    }
}