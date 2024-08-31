use ash::vk;
use crate::vk::VkRenderApi;

impl VkRenderApi {
    pub unsafe fn destroy_pipeline(&self, pipeline: vk::Pipeline) {
        self.device.destroy_pipeline(pipeline, None);
    }
}