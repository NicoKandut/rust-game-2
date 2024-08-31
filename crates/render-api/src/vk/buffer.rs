use ash::vk;
use crate::vk::VkRenderApi;

impl VkRenderApi {
    pub fn create_staging_buffer(&self, size: usize) -> (vk::Buffer, vk::DeviceMemory) {
        self.create_buffer(
            size,
            vk::BufferUsageFlags::TRANSFER_SRC,
            vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
        )
    }

    pub fn create_vertex_buffer(&self, size: usize) -> (vk::Buffer, vk::DeviceMemory) {
        self.create_buffer(
            size,
            vk::BufferUsageFlags::VERTEX_BUFFER | vk::BufferUsageFlags::TRANSFER_DST,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        )
    }

    pub fn create_index_buffer(&self, size: usize) -> (vk::Buffer, vk::DeviceMemory) {
        self.create_buffer(
            size,
            vk::BufferUsageFlags::INDEX_BUFFER | vk::BufferUsageFlags::TRANSFER_DST,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        )
    }

    pub fn create_uniform_buffer(&self, size: usize) -> (vk::Buffer, vk::DeviceMemory) {
        self.create_buffer(
            size,
            vk::BufferUsageFlags::UNIFORM_BUFFER,
            vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
        )
    }

    pub fn create_buffer(&self, size: usize, usage: vk::BufferUsageFlags, properties: vk::MemoryPropertyFlags) -> (vk::Buffer, vk::DeviceMemory) {
        let buffer_create_info = vk::BufferCreateInfo::default()
            .size(size as u64)
            .usage(usage);
        let buffer = unsafe {
            self.device
                .create_buffer(&buffer_create_info, None)
                .unwrap()
        };
        let memory = self.allocate_memory_buffer(buffer, properties);

        unsafe {
            self.device.bind_buffer_memory(buffer, memory, 0)
                .expect("Failed to bind buffer memory!");
        }

        (buffer, memory)
    }

    pub fn create_shader_storage_buffer(&self, size: usize) -> (vk::Buffer, vk::DeviceMemory) {
        self.create_buffer(
            size,
            vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::TRANSFER_DST,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        )
    }

    pub fn destroy_buffer(&self, buffer: vk::Buffer) {
        unsafe { self.device.destroy_buffer(buffer, None); }
    }
}