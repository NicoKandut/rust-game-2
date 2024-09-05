mod descriptor_pool;
mod surface;
mod image_view;
mod descriptor_set;
mod sampler;
mod image_memory_barrier;
mod command_buffer;
mod fence;
mod semaphore;
mod command_pool;
mod framebuffer;
mod shader_module;
mod swapchain;
mod buffer;
mod queue;
mod instance;
mod render_pass;
mod device;
mod physical_device;
mod image;
mod entry;
mod descriptor_set_layout;
mod pipeline;
mod pipeline_layout;
mod memory;

use crate::vk::entry::load_entry;
use crate::vk::instance::create_instance;
use ash::vk;
use ash::vk::{CommandBuffer, DescriptorPool, Fence, PipelineLayout, Semaphore, SwapchainKHR};
use device::create_device;
use physical_device::create_physical_device;
use std::borrow::Cow;
use std::ffi;
use std::ffi::CString;
use vk::{ColorSpaceKHR, CullModeFlags, DebugUtilsMessageSeverityFlagsEXT, DebugUtilsMessageTypeFlagsEXT, DescriptorSetLayout, DeviceMemory, Extent2D, Format, FrontFace, Image, PipelineMultisampleStateCreateInfo, PipelineRasterizationStateCreateInfo, PipelineViewportStateCreateInfo, PolygonMode, PresentModeKHR, PrimitiveTopology, Rect2D, VertexInputRate, Viewport};

pub struct VkRenderApi {
    entry: ash::Entry,
    instance: ash::Instance,
    physical_device: vk::PhysicalDevice,
    device: ash::Device,
    debug_utils_extension: ash::ext::debug_utils::Instance,
    surface_extension: ash::khr::surface::Instance,
    swapchain_extension: ash::khr::swapchain::Device,
    debug_callback: vk::DebugUtilsMessengerEXT,
}

impl VkRenderApi {
    pub fn cmd_bind_descriptor_sets(&self, command_buffer: CommandBuffer, bind_point: vk::PipelineBindPoint, pipeline_layout: PipelineLayout, descriptor_sets: &[vk::DescriptorSet]) {
        unsafe {
            self.device.cmd_bind_descriptor_sets(command_buffer,bind_point, pipeline_layout, 0, descriptor_sets, &[]);
        }
    }
}

impl Default for VkRenderApi {
    fn default() -> Self {
        let entry = load_entry();
        let instance = create_instance(&entry);
        let physical_device = create_physical_device(&instance);
        let device = create_device(&instance);
        let debug_utils_extension = ash::ext::debug_utils::Instance::new(&entry, &instance);
        let surface_extension = ash::khr::surface::Instance::new(&entry, &instance);
        let swapchain_extension = ash::khr::swapchain::Device::new(&instance, &device);
        let debug_callback = create_debug_callback(&debug_utils_extension);

        VkRenderApi {
            entry,
            instance,
            physical_device,
            device,
            debug_utils_extension,
            surface_extension,
            swapchain_extension,
            debug_callback,
        }
    }
}

impl Drop for VkRenderApi {
    fn drop(&mut self) {
        unsafe {
            self.debug_utils_extension.destroy_debug_utils_messenger(self.debug_callback, None);
            self.device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
    }
}

const COLOR_FORMAT: Format = Format::B8G8R8A8_SRGB;
const COLOR_SPACE: ColorSpaceKHR = ColorSpaceKHR::SRGB_NONLINEAR;

impl VkRenderApi {
    pub fn acquire_next_image(&self, swapchain: SwapchainKHR, timeout: u64, semaphore: Semaphore, fence: Fence) -> u32 {
        let (image_index, _) = unsafe {
            self.swapchain_extension.acquire_next_image(swapchain, timeout, semaphore, fence)
                .expect("Failed to acquire next image!")
        };

        image_index
    }

    pub fn transition_image_layout(&self, image: Image, old_layout: vk::ImageLayout, new_layout: vk::ImageLayout, queue: vk::Queue, command_pool: vk::CommandPool) {
        let command_buffer = self.begin_one_time_command_buffer(command_pool);

        let mut barrier = self.create_image_memory_barrier(image, old_layout, new_layout);

        let mut source_stage;
        let mut destination_stage;

        if old_layout == vk::ImageLayout::UNDEFINED && new_layout == vk::ImageLayout::TRANSFER_DST_OPTIMAL {
            barrier = barrier
                .src_access_mask(vk::AccessFlags::empty())
                .dst_access_mask(vk::AccessFlags::TRANSFER_WRITE);
            source_stage = vk::PipelineStageFlags::TOP_OF_PIPE;
            destination_stage = vk::PipelineStageFlags::TRANSFER;
        } else if old_layout == vk::ImageLayout::TRANSFER_DST_OPTIMAL && new_layout == vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL {
            barrier = barrier
                .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
                .dst_access_mask(vk::AccessFlags::SHADER_READ);
            source_stage = vk::PipelineStageFlags::TRANSFER;
            destination_stage = vk::PipelineStageFlags::FRAGMENT_SHADER;
        } else {
            panic!("Unsupported layout transition!");
        }

        self.cmd_pipeline_image_memory_barrier(command_buffer, &[barrier], source_stage, destination_stage);

        self.end_one_time_command_buffer(command_buffer, queue, command_pool);
    }

    pub fn end_one_time_command_buffer(&self, command_buffer: vk::CommandBuffer, queue: vk::Queue, command_pool: vk::CommandPool) {
        self.end_command_buffer(command_buffer);
        let command_buffers = [command_buffer];
        let submit_info = vk::SubmitInfo::default()
            .command_buffers(&command_buffers);
        let submit_infos = [submit_info];
        unsafe {
            self.device.queue_submit(queue, &submit_infos, vk::Fence::null())
                .expect("Failed to submit one-time command buffer!");
        }
        self.queue_wait_idle(queue);
        self.free_command_buffer(command_pool, command_buffer);
    }

    pub fn cmd_pipeline_image_memory_barrier(&self, command_buffer: vk::CommandBuffer, image_memory_barriers: &[vk::ImageMemoryBarrier], src_stage: vk::PipelineStageFlags, dst_stage: vk::PipelineStageFlags) {
        unsafe {
            self.device.cmd_pipeline_barrier(
                command_buffer,
                src_stage,
                dst_stage,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                image_memory_barriers,
            );
        }
    }

    pub fn cmd_bind_index_buffer(&self, command_buffer: vk::CommandBuffer, index_buffer: vk::Buffer) {
        unsafe {
            self.device.cmd_bind_index_buffer(command_buffer, index_buffer, 0, vk::IndexType::UINT32);
        }
    }

    pub fn cmd_bind_vertex_buffer(&self, command_buffer: vk::CommandBuffer, vertex_buffer: vk::Buffer) {
        unsafe {
            let offsets = [0];
            self.device.cmd_bind_vertex_buffers(command_buffer, 0, &[vertex_buffer], &offsets);
        }
    }

    pub fn cmd_copy_buffer_to_image(&self, buffer: vk::Buffer, image: Image, width: u32, height: u32, command_buffer: vk::CommandBuffer) {
        let region = vk::BufferImageCopy::default()
            .buffer_offset(0)
            .buffer_row_length(0)
            .buffer_image_height(0)
            .image_offset(vk::Offset3D { x: 0, y: 0, z: 0 })
            .image_extent(vk::Extent3D { width, height, depth: 1 })
            .image_subresource(vk::ImageSubresourceLayers::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .mip_level(0)
                .base_array_layer(0)
                .layer_count(1)
            );

        unsafe {
            self.device.cmd_copy_buffer_to_image(command_buffer, buffer, image, vk::ImageLayout::TRANSFER_DST_OPTIMAL, &[region]);
        }
    }

    pub fn exec_copy_buffer_to_image(&self, buffer: vk::Buffer, image: Image, width: u32, height: u32, command_pool: vk::CommandPool, queue: vk::Queue) {
        let command_buffer = self.begin_one_time_command_buffer(command_pool);
        self.cmd_copy_buffer_to_image(buffer, image, width, height, command_buffer);
        self.end_one_time_command_buffer(command_buffer, queue, command_pool);
    }

    pub fn present_frame(&self, queue: vk::Queue, swapchain: vk::SwapchainKHR, image_index: usize, wait_semaphore: vk::Semaphore) {
        let swapchains = [swapchain];
        let image_indices = [image_index as u32];
        let wait_semaphores = [wait_semaphore];
        let present_info = vk::PresentInfoKHR::default()
            .wait_semaphores(&wait_semaphores)
            .swapchains(&swapchains)
            .image_indices(&image_indices);

        unsafe {
            self.swapchain_extension.queue_present(queue, &present_info)
                .expect("Failed to present frame!");
        }
    }

    pub fn submit_command_buffers(&self, queue: vk::Queue, command_buffers: &[vk::CommandBuffer], wait_semaphores: &[vk::Semaphore], signal_semaphores: &[vk::Semaphore], fence: vk::Fence) {
        let wait_dst_stage = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let submit_info = vk::SubmitInfo::default()
            .command_buffers(command_buffers)
            .wait_semaphores(wait_semaphores)
            .wait_dst_stage_mask(&wait_dst_stage)
            .signal_semaphores(signal_semaphores);
        let submit_infos = [submit_info];
        unsafe {
            self.device.queue_submit(queue, &submit_infos, fence)
                .expect("Failed to submit command buffer!");
        }
    }

    pub fn submit_command_buffer(&self, queue: vk::Queue, command_buffer: vk::CommandBuffer, wait_semaphores: vk::Semaphore, signal_semaphores: vk::Semaphore, fence: vk::Fence) {
        let command_buffers = [command_buffer];
        let wait_semaphores = [wait_semaphores];
        let signal_semaphores = [signal_semaphores];
        self.submit_command_buffers(queue, &command_buffers, &wait_semaphores, &signal_semaphores, fence);
    }

    pub fn reset_command_buffer(&self, command_buffer: vk::CommandBuffer) {
        unsafe {
            self.device.reset_command_buffer(command_buffer, vk::CommandBufferResetFlags::RELEASE_RESOURCES)
                .expect("Failed to reset command buffer!");
        }
    }

    pub fn begin_one_time_command_buffer(&self, command_pool: vk::CommandPool) -> vk::CommandBuffer {
        let command_buffer = self.create_command_buffer(command_pool);
        self.begin_command_buffer(command_buffer, vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

        command_buffer
    }

    pub fn end_command_buffer(&self, command_buffer: vk::CommandBuffer) {
        unsafe {
            self.device.end_command_buffer(command_buffer)
                .expect("Failed to end command buffer!");
        }
    }

    pub fn begin_command_buffer(&self, command_buffer: vk::CommandBuffer, flags: vk::CommandBufferUsageFlags) {
        let command_buffer_begin_info = vk::CommandBufferBeginInfo::default()
            .flags(flags);
        unsafe {
            self.device.begin_command_buffer(command_buffer, &command_buffer_begin_info)
                .expect("Failed to begin command buffer!");
        }
    }

    pub fn record_copy_command_buffer(&self, command_buffer: vk::CommandBuffer, src: vk::Buffer, dst: vk::Buffer, size: usize) {
        let command_buffer_begin_info = vk::CommandBufferBeginInfo::default()
            .flags(vk::CommandBufferUsageFlags::default());

        unsafe {
            self.device.begin_command_buffer(command_buffer, &command_buffer_begin_info)
                .expect("Failed to begin command buffer!");
        }

        self.cmd_copy_buffer(src, dst, size, command_buffer);

        unsafe {
            self.device.end_command_buffer(command_buffer)
                .expect("Failed to end command buffer!");
        }
    }

    pub fn do_render_pass(&self, command_buffer: vk::CommandBuffer, pipeline: vk::Pipeline, render_pass: vk::RenderPass, frame_buffer: vk::Framebuffer, width: u32, height: u32, index_count: u32) {
        let clear_values = [
            vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [0.0, 0.0, 0.0, 1.0],
                },
            },
        ];

        let render_pass_begin_info = vk::RenderPassBeginInfo::default()
            .render_pass(render_pass)
            .framebuffer(frame_buffer)
            .render_area(vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: vk::Extent2D { width, height },
            })
            .clear_values(&clear_values);

        unsafe {
            self.device.cmd_begin_render_pass(command_buffer, &render_pass_begin_info, vk::SubpassContents::INLINE);
            self.device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, pipeline);
            self.device.cmd_draw_indexed(command_buffer, index_count, 1, 0, 0, 0);
            self.device.cmd_end_render_pass(command_buffer);
        }
    }

    pub fn create_graphics_pipeline(&self, vertex_shader_module: vk::ShaderModule, fragment_shader_module: vk::ShaderModule, render_pass: vk::RenderPass, descriptor_set_layout: vk::DescriptorSetLayout) -> (vk::Pipeline, vk::PipelineLayout, DescriptorSetLayout) {
        let main = CString::new("main").unwrap();
        let vertex_shader_stage_create_info = vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlags::VERTEX)
            .module(vertex_shader_module)
            .name(main.as_c_str());
        let fragment_shader_stage_create_info = vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlags::FRAGMENT)
            .module(fragment_shader_module)
            .name(main.as_c_str());
        let shader_stages = [vertex_shader_stage_create_info, fragment_shader_stage_create_info];
        let (vertex_binding, vertex_attribute) = Self::get_vertex_input_description();
        let vertex_bindings = [vertex_binding];
        let vertex_attributes = [vertex_attribute];
        let vertex_input_state_create_info = vk::PipelineVertexInputStateCreateInfo::default()
            .vertex_binding_descriptions(&vertex_bindings)
            .vertex_attribute_descriptions(&vertex_attributes);
        let input_assembly_state_create_info = vk::PipelineInputAssemblyStateCreateInfo::default()
            .topology(PrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false);
        let viewport = Viewport::default()
            .width(600.0)
            .height(300.0)
            .max_depth(1.0);
        let scissor = Rect2D::default()
            .extent(Extent2D { width: 600, height: 300 });
        let viewports = [viewport];
        let scissors = [scissor];
        let viewport_state_create_info = PipelineViewportStateCreateInfo::default()
            .viewports(&viewports)
            .scissors(&scissors);
        let rasterization_state_create_info = PipelineRasterizationStateCreateInfo::default()
            .depth_clamp_enable(false)
            .rasterizer_discard_enable(false)
            .polygon_mode(PolygonMode::FILL)
            .line_width(1.0)
            .cull_mode(CullModeFlags::BACK)
            .front_face(FrontFace::CLOCKWISE)
            .depth_bias_enable(false);
        let multisample_state_create_info = PipelineMultisampleStateCreateInfo::default()
            .sample_shading_enable(false)
            .rasterization_samples(vk::SampleCountFlags::TYPE_1);
        let color_blend_attachment = vk::PipelineColorBlendAttachmentState::default()
            .color_write_mask(vk::ColorComponentFlags::RGBA)
            .blend_enable(false);
        let color_blend_attachments = [color_blend_attachment];
        let color_blend_state_create_info = vk::PipelineColorBlendStateCreateInfo::default()
            .attachments(&color_blend_attachments);

        let layouts = [descriptor_set_layout];
        let layout_info = vk::PipelineLayoutCreateInfo::default()
            .set_layouts(&layouts);
        let pipeline_layout = unsafe {
            self.device.create_pipeline_layout(&layout_info, None)
                .expect("Failed to create pipeline layout!")
        };

        let pipeline_create_info = vk::GraphicsPipelineCreateInfo::default()
            .stages(&shader_stages)
            .vertex_input_state(&vertex_input_state_create_info)
            .input_assembly_state(&input_assembly_state_create_info)
            .viewport_state(&viewport_state_create_info)
            .rasterization_state(&rasterization_state_create_info)
            .multisample_state(&multisample_state_create_info)
            .color_blend_state(&color_blend_state_create_info)
            .layout(pipeline_layout)
            .render_pass(render_pass)
            .subpass(0);

        let pipeline_create_infos = [pipeline_create_info];
        let pipeline = unsafe {
            self.device.create_graphics_pipelines(vk::PipelineCache::null(), &pipeline_create_infos, None)
                .expect("Failed to create graphics pipeline!")
                .pop()
                .expect("Failed to get graphics pipeline!")
        };

        (pipeline, pipeline_layout, DescriptorSetLayout::null())
    }

    pub fn create_compute_pipeline(&self, compute_shader_module: vk::ShaderModule, descriptor_set_layout: vk::DescriptorSetLayout) -> (vk::Pipeline, vk::PipelineLayout) {
        let entry_point_name = CString::new("main").unwrap();
        let vertex_shader_state_info = vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlags::COMPUTE)
            .module(compute_shader_module)
            .name(&entry_point_name);

        let pipeline_layout = {
            let layouts = [descriptor_set_layout];
            let layout_info = vk::PipelineLayoutCreateInfo::default()
                .set_layouts(&layouts);
            // .push_constant_range;

            unsafe { self.device.create_pipeline_layout(&layout_info, None).unwrap() }
        };

        let pipeline_info = vk::ComputePipelineCreateInfo::default()
            .stage(vertex_shader_state_info)
            .layout(pipeline_layout);
        let pipeline_infos = [pipeline_info];

        let pipeline = unsafe {
            self.device
                .create_compute_pipelines(vk::PipelineCache::null(), &pipeline_infos, None)
                .unwrap()[0]
        };

        unsafe {
            self.device.destroy_shader_module(compute_shader_module, None);
        };

        (pipeline, pipeline_layout)
    }


    pub fn get_vertex_input_description() -> (vk::VertexInputBindingDescription, vk::VertexInputAttributeDescription) {
        let binding = vk::VertexInputBindingDescription::default()
            .binding(0)
            .stride(size_of::<f32>() as u32 * 4)
            .input_rate(VertexInputRate::VERTEX);

        let attribute = vk::VertexInputAttributeDescription::default()
            .binding(0)
            .location(0)
            .format(Format::R32G32B32A32_SFLOAT)
            .offset(0);

        (binding, attribute)
    }


    fn get_surface_format(&self, surface: vk::SurfaceKHR) -> (Format, ColorSpaceKHR) {
        let formats = unsafe {
            self.surface_extension.get_physical_device_surface_formats(self.physical_device.clone(), surface)
                .expect("Failed to get surface formats!")
        };
        assert!(!formats.is_empty(), "No surface formats available!");

        let preferred_format = COLOR_FORMAT;
        let preferred_color_space = COLOR_SPACE;

        for format in formats.iter() {
            if format.format == preferred_format && format.color_space == preferred_color_space {
                return (preferred_format, preferred_color_space);
            }
        }

        (formats[0].format, formats[0].color_space)
    }

    fn get_present_mode(&self, surface: vk::SurfaceKHR) -> PresentModeKHR {
        let present_modes = unsafe {
            self.surface_extension.get_physical_device_surface_present_modes(self.physical_device.clone(), surface)
                .expect("Failed to get present modes!")
        };
        assert!(!present_modes.is_empty(), "No present modes available!");

        let preferred_mode = PresentModeKHR::IMMEDIATE;

        for present_mode in present_modes {
            if present_mode == preferred_mode {
                return preferred_mode;
            }
        }

        PresentModeKHR::FIFO
    }


    pub fn copy_to_device_memory(&self, data: &[u8], device_memory: DeviceMemory) {
        unsafe {
            let data_ptr = self.device.map_memory(device_memory, 0, data.len() as u64, vk::MemoryMapFlags::empty())
                .expect("Failed to map memory!") as *mut u8;
            data_ptr.copy_from_nonoverlapping(data.as_ptr(), data.len());
            self.device.unmap_memory(device_memory);
        }
    }

    pub fn cmd_copy_buffer(&self, src: vk::Buffer, dst: vk::Buffer, size: usize, command_buffer: vk::CommandBuffer) {
        let region = vk::BufferCopy {
            src_offset: 0,
            dst_offset: 0,
            size: size as u64,
        };
        unsafe {
            self.device.cmd_copy_buffer(command_buffer, src, dst, &[region]);
        }
    }
}


fn create_debug_callback(debug_utils_extension: &ash::ext::debug_utils::Instance) -> vk::DebugUtilsMessengerEXT {
    let debug_info = vk::DebugUtilsMessengerCreateInfoEXT::default()
        .message_severity(
            DebugUtilsMessageSeverityFlagsEXT::ERROR
                | DebugUtilsMessageSeverityFlagsEXT::WARNING
                | DebugUtilsMessageSeverityFlagsEXT::INFO,
        )
        .message_type(
            DebugUtilsMessageTypeFlagsEXT::GENERAL
                | DebugUtilsMessageTypeFlagsEXT::VALIDATION
                | DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
        )
        .pfn_user_callback(Some(vulkan_debug_callback));

    unsafe {
        debug_utils_extension
            .create_debug_utils_messenger(&debug_info, None)
            .unwrap()
    }
}

unsafe extern "system" fn vulkan_debug_callback(message_severity: vk::DebugUtilsMessageSeverityFlagsEXT, message_type: vk::DebugUtilsMessageTypeFlagsEXT, p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT<'_>, _user_data: *mut std::os::raw::c_void) -> vk::Bool32 {
    let callback_data = *p_callback_data;
    let message_id_number = callback_data.message_id_number;

    let message_id_name = if callback_data.p_message_id_name.is_null() {
        Cow::from("")
    } else {
        ffi::CStr::from_ptr(callback_data.p_message_id_name).to_string_lossy()
    };

    let message = if callback_data.p_message.is_null() {
        Cow::from("")
    } else {
        ffi::CStr::from_ptr(callback_data.p_message).to_string_lossy()
    };

    println!(
        "[DEBUG] {message_severity:?}:\n{message_type:?} [{message_id_name} ({message_id_number})] : {message}\n",
    );

    vk::FALSE
}