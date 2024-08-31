use ash::vk;
use crate::vk::VkRenderApi;

impl VkRenderApi {
    pub fn create_semaphore(&self) -> vk::Semaphore {
        let semaphore_create_info = vk::SemaphoreCreateInfo::default();
        unsafe {
            self.device.create_semaphore(&semaphore_create_info, None)
                .expect("Failed to create semaphore!")
        }
    }

    pub fn destroy_semaphore(&self, semaphore: vk::Semaphore) {
        unsafe {
            self.device.destroy_semaphore(semaphore, None);
        }
    }

    pub fn destroy_semaphores(&self, semaphore: Vec<vk::Semaphore>) {
        for semaphore in semaphore {
            self.destroy_semaphore(semaphore);
        }
    }
}