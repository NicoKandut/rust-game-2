use ash::vk;
use ash::vk::{DeviceMemory, Image, MemoryPropertyFlags};
use crate::vk::VkRenderApi;

impl VkRenderApi {
    fn find_memory_type(&self, properties: MemoryPropertyFlags) -> u32 {
        let memory_properties = unsafe { self.instance.get_physical_device_memory_properties(self.physical_device.clone()) };
        for i in 0..memory_properties.memory_type_count {
            if (memory_properties.memory_types[i as usize].property_flags & properties) == properties {
                return i;
            }
        }

        panic!("Failed to find suitable memory type!");
    }

    pub(crate) fn allocate_memory_buffer(
        &self,
        buffer: vk::Buffer,
        properties: MemoryPropertyFlags,
    ) -> DeviceMemory {
        let memory_requirements = unsafe { self.device.get_buffer_memory_requirements(buffer) };
        self.allocate_memory(memory_requirements, properties)
    }

    pub(crate) fn allocate_memory_image(
        &self,
        image: Image,
        properties: MemoryPropertyFlags,
    ) -> DeviceMemory {
        let memory_requirements = unsafe { self.device.get_image_memory_requirements(image) };
        self.allocate_memory(memory_requirements, properties)
    }

    fn allocate_memory(&self, requirements: vk::MemoryRequirements, properties: MemoryPropertyFlags) -> DeviceMemory {
        let memory_type_index = self.find_memory_type(properties);
        let memory_allocate_info = vk::MemoryAllocateInfo {
            allocation_size: requirements.size,
            memory_type_index,
            ..Default::default()
        };

        unsafe {
            self.device
                .allocate_memory(&memory_allocate_info, None)
                .unwrap()
        }
    }

    pub fn free_memory(&self, memory: vk::DeviceMemory) {
        unsafe { self.device.free_memory(memory, None); }
    }

    pub fn free_memories(&self, memory: Vec<vk::DeviceMemory>) {
        for memory in memory {
            self.free_memory(memory);
        }
    }
}