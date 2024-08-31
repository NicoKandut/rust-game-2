use ash::vk;
use crate::vk::VkRenderApi;

impl VkRenderApi {
    pub fn create_command_pool(&self, queue_family_index: u32) -> vk::CommandPool {
        let command_pool_create_info = vk::CommandPoolCreateInfo::default()
            .queue_family_index(queue_family_index)
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);

        unsafe {
            self.device.create_command_pool(&command_pool_create_info, None)
                .expect("Failed to create command pool!")
        }
    }

    pub fn destroy_command_pool(&self, command_pool: vk::CommandPool) {
        unsafe {
            self.device.destroy_command_pool(command_pool, None);
        }
    }
}