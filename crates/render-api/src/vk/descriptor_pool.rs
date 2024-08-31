use ash::vk;
use crate::vk::VkRenderApi;

impl VkRenderApi {
    pub fn create_descriptor_pool(&self, pool_sizes: &[vk::DescriptorPoolSize]) -> vk::DescriptorPool {
        let pool_info = vk::DescriptorPoolCreateInfo::default()
            .pool_sizes(&pool_sizes)
            .max_sets(2);

        unsafe {
            self.device.create_descriptor_pool(&pool_info, None)
                .expect("Failed to create descriptor pool!")
        }
    }

    pub fn destroy_descriptor_pool(&self, descriptor_pool: vk::DescriptorPool) {
        unsafe {
            self.device.destroy_descriptor_pool(descriptor_pool, None);
        }
    }
}