use std::{ptr::null, sync::Arc};

use ash::vk::{self, ImageLayout, InstanceCreateInfo, SampleCountFlags, SharingMode, SurfaceKHR};
use ash::vk::{Extent3D, ImageCreateInfo, ImageTiling, ImageType};
use vk::{ImageUsageFlags, MemoryPropertyFlags};

pub struct VkRenderApi {
    entry: Arc<ash::Entry>,
    instance: Arc<ash::Instance>,
    physical_device: Arc<vk::PhysicalDevice>,
    device: Arc<ash::Device>,
}

impl Default for VkRenderApi {
    fn default() -> Self {
        let entry = load_entry();
        let instance = create_instance(&entry);
        let physical_device = create_physical_device(&instance);
        let device = create_device(&instance);

        VkRenderApi {
            entry: Arc::new(entry),
            instance: Arc::new(instance),
            physical_device: Arc::new(physical_device),
            device: Arc::new(device),
        }
    }
}

impl Drop for VkRenderApi {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
    }
}

impl VkRenderApi {
    pub(crate) fn create_buffer(&self, usage: vk::BufferUsageFlags, size: u64) -> (vk::Buffer, vk::DeviceMemory) {
        let buffer_create_info = vk::BufferCreateInfo {
            size,
            usage,
            ..Default::default()
        };

        let buffer = unsafe {
            self.device
                .create_buffer(&buffer_create_info, None)
                .unwrap()
        };

        let memory = self.allocate_memory_buffer(
            buffer,
            MemoryPropertyFlags::DEVICE_LOCAL,
        );

        (buffer, memory)
    }

    // pub(crate) fn create_image(&self, width: u32, height: u32) -> (vk::Image, vk::DeviceMemory) {
    //     let create_info = ImageCreateInfo::builder()
    //         .image_type(ImageType::TYPE_2D)
    //         .format(vk::Format::R8G8B8_UNORM)
    //         .extent(Extent3D { width, height, depth: 1 })
    //         .mip_levels(1)
    //         .array_layers(1)
    //         .samples(SampleCountFlags::TYPE_1)
    //         .tiling(ImageTiling::OPTIMAL)
    //         .usage(ImageUsageFlags::STORAGE | ImageUsageFlags::SAMPLED)
    //         .sharing_mode(SharingMode::EXCLUSIVE)
    //         .initial_layout(ImageLayout::UNDEFINED)
    //         .build();
    //
    //     let image = unsafe { self.device.create_image(&create_info, None).unwrap() };
    //
    //     let memory = self.allocate_memory_image(
    //         image,
    //         MemoryPropertyFlags::DEVICE_LOCAL,
    //     );
    //
    //     (image, memory)
    // }

    // pub(crate) fn copy_to_buffer(&self, buffer: vk::Buffer, data: &[u8]) {
    //     let buffer_size = data.len() as u64;
    //     let (staging_buffer, staging_buffer_memory) = self.create_buffer(vk::BufferUsageFlags::TRANSFER_SRC, buffer_size);
    //
    //     unsafe {
    //         let data_ptr = self.device.map_memory(
    //             staging_buffer_memory,
    //             0,
    //             buffer_size,
    //             vk::MemoryMapFlags::empty(),
    //         ).unwrap() as *mut u8;
    //         data_ptr.copy_from_nonoverlapping(data.as_ptr(), data.len());
    //         self.device.unmap_memory(staging_buffer_memory);
    //     }
    //
    //     unsafe {
    //         self.device.destroy_buffer(staging_buffer, None);
    //         self.device.free_memory(staging_buffer_memory, None);
    //     }
    // }

    pub fn create_surface(&self, window: &glfw::Window) -> SurfaceKHR {
        let mut surface = SurfaceKHR::null();
        window
            .create_window_surface(self.instance.handle(), null(), &mut surface)
            .result()
            .unwrap();
        surface
    }

    pub fn create_queue(&self) -> vk::Queue {
        let queue_family_properties = unsafe {
            self.instance
                .get_physical_device_queue_family_properties(*self.physical_device.clone())
        };
        let queue_family_index = queue_family_properties
            .iter()
            .position(|queue_family_properties| {
                queue_family_properties
                    .queue_flags
                    .contains(vk::QueueFlags::GRAPHICS | ash::vk::QueueFlags::COMPUTE)
            })
            .unwrap() as u32;

        unsafe { self.device.get_device_queue(queue_family_index, 0) }
    }

    pub fn free_memory(&self, memory: vk::DeviceMemory) {
        unsafe { self.device.free_memory(memory, None); }
    }

    pub fn drop_image(&self, image: vk::Image) {
        unsafe { self.device.destroy_image(image, None); }
    }

    fn find_memory_type(&self, properties: MemoryPropertyFlags) -> u32 {
        let memory_properties = unsafe { self.instance.get_physical_device_memory_properties(*self.physical_device.clone()) };
        for i in 0..memory_properties.memory_type_count {
            if (memory_properties.memory_types[i as usize].property_flags & properties) == properties {
                return i;
            }
        }

        panic!("Failed to find suitable memory type!");
    }

    fn allocate_memory_buffer(
        &self,
        buffer: vk::Buffer,
        properties: MemoryPropertyFlags,
    ) -> vk::DeviceMemory {
        let memory_requirements = unsafe { self.device.get_buffer_memory_requirements(buffer) };
        self.allocate_memory(memory_requirements, properties)
    }

    fn allocate_memory_image(
        &self,
        image: vk::Image,
        properties: MemoryPropertyFlags,
    ) -> vk::DeviceMemory {
        let memory_requirements = unsafe { self.device.get_image_memory_requirements(image) };
        self.allocate_memory(memory_requirements, properties)
    }

    fn allocate_memory(
        &self,
        requirements: vk::MemoryRequirements,
        properties: MemoryPropertyFlags,
    ) -> vk::DeviceMemory {
        let memory_type_index = self.find_memory_type(properties); // TODO: obtain from physical device
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
}



fn load_entry() -> ash::Entry {
    unsafe { ash::Entry::load().unwrap() }
}

fn create_instance(entry: &ash::Entry) -> ash::Instance {
    let extension_names = vec![
        ash::ext::debug_utils::NAME.as_ptr(),
        ash::khr::surface::NAME.as_ptr(),
        ash::khr::win32_surface::NAME.as_ptr(),
    ];
    let instance_create_info = InstanceCreateInfo::default()
        .enabled_extension_names(&extension_names);

    unsafe {
        let instance = entry
            .create_instance(&instance_create_info, Option::None)
            .unwrap();
        instance
    }
}

fn create_physical_device(instance: &ash::Instance) -> ash::vk::PhysicalDevice {
    let physical_devices = unsafe { instance.enumerate_physical_devices().unwrap() };
    physical_devices[0]
}

fn create_device(instance: &ash::Instance) -> ash::Device {
    let physical_devices = unsafe { instance.enumerate_physical_devices().unwrap() };
    let physical_device = physical_devices[0];

    let queue_family_properties =
        unsafe { instance.get_physical_device_queue_family_properties(physical_device) };
    let queue_family_index = queue_family_properties
        .iter()
        .position(|queue_family_properties| {
            queue_family_properties
                .queue_flags
                .contains(ash::vk::QueueFlags::GRAPHICS)
        })
        .unwrap() as u32;

    let device_queue_create_info = ash::vk::DeviceQueueCreateInfo {
        queue_family_index,
        queue_count: 1,
        p_queue_priorities: [1.0].as_ptr(),
        ..Default::default()
    };

    let device_create_info = ash::vk::DeviceCreateInfo {
        queue_create_info_count: 1,
        p_queue_create_infos: &device_queue_create_info,
        ..Default::default()
    };

    unsafe {
        instance
            .create_device(physical_device, &device_create_info, Option::None)
            .unwrap()
    }
}
