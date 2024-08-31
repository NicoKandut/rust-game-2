use ash::vk;
use crate::vk::VkRenderApi;

impl VkRenderApi {
    pub fn create_descriptor_set_layout(&self, bindings: &[vk::DescriptorSetLayoutBinding]) -> vk::DescriptorSetLayout {
        let layout_info = vk::DescriptorSetLayoutCreateInfo::default()
            .bindings(bindings);

        unsafe {
            self.device
                .create_descriptor_set_layout(&layout_info, None)
                .unwrap()
        }
    }

    pub fn destroy_descriptor_set_layout(&self, layout: vk::DescriptorSetLayout) {
        unsafe {
            self.device.destroy_descriptor_set_layout(layout, None);
        }
    }
}