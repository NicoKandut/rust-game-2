use ash::vk;
use crate::vk::VkRenderApi;

pub(crate) fn create_device(instance: &ash::Instance) -> ash::Device {
    let physical_devices = unsafe { instance.enumerate_physical_devices().unwrap() };
    let physical_device = physical_devices.iter()
        .cloned()
        .map(|physical_device| {
            let properties = unsafe { instance.get_physical_device_properties(physical_device) };
            let mut score = 0;
            if properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU {
                score += 1000;
            }
            (physical_device, score)
        })
        .max_by_key(|x| x.1)
        .expect("Failed to find a suitable physical device!")
        .0;


    let queue_family_properties = unsafe {
        instance.get_physical_device_queue_family_properties(physical_device)
    };
    let queue_family_index = queue_family_properties
        .iter()
        .position(|queue_family_properties| {
            queue_family_properties
                .queue_flags
                .contains(vk::QueueFlags::COMPUTE | vk::QueueFlags::GRAPHICS)
        })
        .unwrap() as u32;

    let device_queue_create_info = ash::vk::DeviceQueueCreateInfo {
        queue_family_index,
        queue_count: 1,
        p_queue_priorities: [1.0].as_ptr(),
        ..Default::default()
    };

    let extension_names = &[ash::khr::swapchain::NAME.as_ptr()];
    let queue_create_infos = &[device_queue_create_info];
    let device_create_info = ash::vk::DeviceCreateInfo::default()
        .enabled_extension_names(extension_names)
        .queue_create_infos(queue_create_infos);

    unsafe {
        instance
            .create_device(physical_device, &device_create_info, None)
            .unwrap()
    }
}

impl VkRenderApi {
    pub fn device_wait_idle(&self) {
        unsafe {
            self.device.device_wait_idle().expect("Failed to wait for device idle!");
        }
    }
}