use ash::vk;
use crate::vk::VkRenderApi;

impl VkRenderApi {
    pub fn destroy_pipeline_layout(&self, layout: vk::PipelineLayout) {
        unsafe {
            self.device.destroy_pipeline_layout(layout, None);
        }
    }
}