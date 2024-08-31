use ash::vk;
use crate::vk::VkRenderApi;

impl VkRenderApi {
    pub fn create_command_buffer(&self, command_pool: vk::CommandPool) -> vk::CommandBuffer {
        self.create_command_buffers(command_pool, 1)
            .pop()
            .expect("Failed to get command buffer!")
    }

    pub fn create_command_buffers(&self, command_pool: vk::CommandPool, count: usize) -> Vec<vk::CommandBuffer> {
        let command_buffer_allocate_info = vk::CommandBufferAllocateInfo::default()
            .command_pool(command_pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(count as u32);

        unsafe {
            self.device.allocate_command_buffers(&command_buffer_allocate_info)
                .expect("Failed to allocate command buffer!")
        }
    }

    pub fn free_command_buffers(&self, command_pool: vk::CommandPool, command_buffers: &[vk::CommandBuffer; 1]) {
        unsafe {
            self.device.free_command_buffers(command_pool, command_buffers);
        }
    }

    pub fn free_command_buffer(&self, command_pool: vk::CommandPool, command_buffers: vk::CommandBuffer) {
        let command_buffers = [command_buffers];
        self.free_command_buffers(command_pool, &command_buffers);
    }
}