use ash::vk;
use crate::vk::VkRenderApi;

impl VkRenderApi {

    pub fn allocate_descriptor_sets(&self, descriptor_pool: vk::DescriptorPool, descriptor_set_layouts: &[vk::DescriptorSetLayout]) -> Vec<vk::DescriptorSet> {
        let allocate_info = vk::DescriptorSetAllocateInfo::default()
            .descriptor_pool(descriptor_pool)
            .set_layouts(descriptor_set_layouts);

        unsafe {
            self.device
                .allocate_descriptor_sets(&allocate_info)
                .expect("Failed to allocate descriptor sets!")
        }
    }

    pub fn update_descriptor_sets(&self, descriptor_writes: &[vk::WriteDescriptorSet]) {
        unsafe {
            self.device.update_descriptor_sets(descriptor_writes, &[]);
        }
    }
}
