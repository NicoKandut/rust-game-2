use ash::vk;

pub(crate) fn create_physical_device(instance: &ash::Instance) -> vk::PhysicalDevice {
    let physical_devices = unsafe { instance.enumerate_physical_devices().unwrap() };
    physical_devices[0]
}