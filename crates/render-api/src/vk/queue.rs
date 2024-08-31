use ash::vk;
use crate::vk::VkRenderApi;

impl VkRenderApi {
    pub fn create_queue(&self, flags: vk::QueueFlags) -> (u32, vk::Queue) {
        let queue_family_properties = unsafe {
            self.instance.get_physical_device_queue_family_properties(self.physical_device.clone())
        };
        let queue_family_index = queue_family_properties
            .iter()
            .position(|queue_family_properties| {
                queue_family_properties.queue_flags.contains(flags)
            })
            .unwrap() as u32;

        let queue = unsafe {
            self.device.get_device_queue(queue_family_index, 0)
        };

        (queue_family_index, queue)
    }

    pub fn create_present_queue(&self, surface: vk::SurfaceKHR) -> (u32, vk::Queue) {
        let queue_family_properties = unsafe {
            self.instance
                .get_physical_device_queue_family_properties(self.physical_device.clone())
        };
        let queue_family_index = queue_family_properties
            .iter()
            .enumerate()
            .position(|(index, _queue_family_properties)| {
                unsafe {
                    ash::khr::surface::Instance::new(&self.entry, &self.instance)
                        .get_physical_device_surface_support(self.physical_device.clone(), index as u32, surface)
                        .unwrap()
                }
            })
            .unwrap() as u32;

        let queue = unsafe { self.device.get_device_queue(queue_family_index, 0) };

        (queue_family_index, queue)
    }

    pub fn queue_wait_idle(&self, queue: vk::Queue) {
        unsafe {
            self.device.queue_wait_idle(queue)
                .expect("Failed to wait for queue idle!");
        }
    }
}