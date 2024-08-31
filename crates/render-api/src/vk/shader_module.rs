use ash::vk;
use crate::vk::VkRenderApi;

impl VkRenderApi {
    pub fn create_shader_module(&self, vertex_shader_code: &[u32]) -> vk::ShaderModule {
        let shader_module_create_info = vk::ShaderModuleCreateInfo::default()
            .code(vertex_shader_code);
        unsafe {
            self.device.create_shader_module(&shader_module_create_info, None)
                .expect("Failed to create shader module!")
        }
    }

    pub fn destroy_shader_module(&self, shader_module: vk::ShaderModule) {
        unsafe { self.device.destroy_shader_module(shader_module, None); }
    }
}