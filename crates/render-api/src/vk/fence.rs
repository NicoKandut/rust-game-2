use ash::vk;
use crate::vk::VkRenderApi;

impl VkRenderApi {
    pub fn create_fence(&self, flags: vk::FenceCreateFlags) -> vk::Fence {
        let fence_create_info = vk::FenceCreateInfo::default()
            .flags(flags);
        unsafe {
            self.device.create_fence(&fence_create_info, None)
                .expect("Failed to create fence!")
        }
    }

    pub fn wait_for_fences(&self, fences: &[vk::Fence]) {
        unsafe {
            self.device.wait_for_fences(fences, true, u64::MAX)
                .expect("Failed to wait for fences!");
        }
    }

    pub fn wait_for_fence(&self, fence: vk::Fence) {
        let fences = [fence];
        self.wait_for_fences(&fences);
    }

    pub fn reset_fences(&self, fence: &[vk::Fence; 1]) {
        unsafe {
            self.device.reset_fences(fence)
                .expect("Failed to reset fences!");
        }
    }

    pub fn reset_fence(&self, fence: vk::Fence) {
        let fences = [fence];
        self.reset_fences(&fences);
    }

    pub fn destroy_fence(&self, fence: vk::Fence) {
        unsafe {
            self.device.destroy_fence(fence, None);
        }
    }

    pub fn destroy_fences(&self, fences: Vec<vk::Fence>) {
        for fence in fences {
            self.destroy_fence(fence);
        }
    }
}