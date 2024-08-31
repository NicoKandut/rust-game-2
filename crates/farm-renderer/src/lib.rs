mod uniform_buffer_object;

use ash::vk;
use ash::vk::{DescriptorSetLayout, Framebuffer, ImageView, Pipeline, PipelineLayout, Queue, QueueFlags, ShaderModule, SurfaceKHR, SwapchainKHR};
use core::slice::from_raw_parts;
use glfw::PWindow;
use image::DynamicImage;
use linalg::{Identity, Matrix4, Vector3};
use rand::Rng;
use render_api::vk::VkRenderApi;
use shader::{get_farm_fragment_shader, get_farm_vertex_shader};
use uniform_buffer_object::UniformBufferObject;
use vk::RenderPass;
use world::world::World;

pub struct Renderer {
    api: VkRenderApi,
    surface: SurfaceKHR,
    width: usize,
    height: usize,
    graphics_queue: Queue,
    present_queue: Queue,
    vertex_shader_module: ShaderModule,
    fragment_shader_module: ShaderModule,
    render_pass: RenderPass,
    pipeline: Pipeline,
    pipeline_layout: PipelineLayout,
    descriptor_set_layout: DescriptorSetLayout,

    vertex_buffer: vk::Buffer,
    vertex_memory: vk::DeviceMemory,
    index_buffer: vk::Buffer,
    index_memory: vk::DeviceMemory,

    // world: [[u32; 128]; 128],
    // world_changed: bool,
    // world_buffer: Buffer,
    // world_memory: DeviceMemory,
    // world_staging_buffer: Buffer,
    // world_staging_memory: DeviceMemory,

    texture: DynamicImage,
    texture_image_buffer: vk::Image,
    texture_image_memory: vk::DeviceMemory,
    texture_image_view: ImageView,

    swapchain: SwapchainKHR,
    swapchain_image_views: Vec<ImageView>,
    frame_buffers: Vec<Framebuffer>,
    command_pool: vk::CommandPool,
    command_buffers: Vec<vk::CommandBuffer>,
    uniform_buffers: Vec<vk::Buffer>,
    uniform_memories: Vec<vk::DeviceMemory>,

    image_available_semaphores: Vec<vk::Semaphore>,
    render_finished_semaphores: Vec<vk::Semaphore>,
    in_flight_fences: Vec<vk::Fence>,
    frame_index: usize,
    quad_vertices: Vec<Vector3>,
    quad_indices: Vec<u32>,
    world_changed: bool,
    texture_changed: bool,
    texture_sampler: vk::Sampler,
    descriptor_pool: vk::DescriptorPool,
    descriptor_sets: Vec<vk::DescriptorSet>,

    uniform_buffer_object: UniformBufferObject,
}

impl Renderer {
    pub fn new(p_window: &PWindow) -> Self {
        let api = VkRenderApi::default();

        let surface = api.create_surface(p_window);
        let (graphics_queue_index, graphics_queue) = api.create_queue(QueueFlags::GRAPHICS);
        let (present_queue_index, present_queue) = api.create_present_queue(surface);

        let (width, height) = p_window.get_size();
        let width = width as usize;
        let height = height as usize;

        let mut rng = rand::thread_rng();

        let cube_vertices = vec![
            Vector3::new(1.0, 1.0, 0.0),
            Vector3::new(-1.0, -1.0, 0.0),
            Vector3::new(1.0, -1.0, 0.0),
            Vector3::new(-1.0, 1.0, 0.0),
        ];
        let cube_indices = vec![
            0, 1, 2,
            2, 1, 0,
            1, 2, 3,
            3, 2, 1,
        ];

        let mut quad_vertices = Vec::<Vector3>::with_capacity(1000);
        let mut quad_indices = Vec::<u32>::with_capacity(3 * 1000);

        // // RNG
        // for _ in 0..quad_vertices.capacity() {
        //     quad_vertices.push(Vector3::new(
        //         rng.gen_range(-1.0..1.0),
        //         rng.gen_range(-1.0..1.0),
        //         rng.gen_range(-1.0..1.0)
        //     ));
        // }
        // for _ in 0..quad_indices.capacity() {
        //     quad_indices.push(rng.gen_range(0..quad_vertices.len() as u32));
        // }

        // CUBE
        for v in cube_vertices {
            quad_vertices.push(v);
        }
        for i in cube_indices {
            quad_indices.push(i);
        }

        let quad_vertex_buffer_size = quad_vertices.len() * 2 * size_of::<f32>();
        let (quad_vertex_buffer, quad_vertex_memory) = api.create_vertex_buffer(quad_vertex_buffer_size);

        let quad_index_buffer_size = quad_indices.len() * size_of::<u32>();
        let (quad_index_buffer, quad_index_memory) = api.create_index_buffer(quad_index_buffer_size);

        let texture = images::field_empty();
        let (texture_image_buffer, texture_image_memory) = api.create_texture_image(texture.width(), texture.height());
        let texture_image_view = api.create_image_view(texture_image_buffer);
        let texture_sampler = api.create_sampler();

        let (swapchain_extension, swapchain) = api.create_swapchain(surface, width as usize, height as usize);

        let swapchain_images = unsafe {
            swapchain_extension.get_swapchain_images(swapchain).expect("Failed to get swapchain images")
        };

        let swapchain_image_views = api.create_swapchain_images_views(swapchain_images);

        let vertex_shader_code = get_farm_vertex_shader();
        let fragment_shader_code = get_farm_fragment_shader();
        let vertex_shader_module = api.create_shader_module(&vertex_shader_code);
        let fragment_shader_module = api.create_shader_module(&fragment_shader_code);

        let render_pass = api.create_render_pass();

        let uniform_size = size_of::<UniformBufferObject>();
        let mut uniform_buffers = vec![];
        let mut uniform_memories = vec![];
        for _ in 0..2 {
            let (uniform_buffer, uniform_memory) = api.create_uniform_buffer(uniform_size);
            uniform_buffers.push(uniform_buffer);
            uniform_memories.push(uniform_memory);
        }

        let pool_sizes = [
            vk::DescriptorPoolSize::default()
                .ty(vk::DescriptorType::UNIFORM_BUFFER)
                .descriptor_count(2),
        ];

        let descriptor_pool = api.create_descriptor_pool(&pool_sizes);

        let bindings = [
            vk::DescriptorSetLayoutBinding::default()
                .binding(0)
                .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::VERTEX)
        ];
        let descriptor_set_layout = api.create_descriptor_set_layout(&bindings);
        let descriptor_set_layouts = [descriptor_set_layout, descriptor_set_layout];

        let descriptor_sets = api.allocate_descriptor_sets(descriptor_pool, &descriptor_set_layouts);

        for i in 0..2 {
            let descriptor_buffer_info = vk::DescriptorBufferInfo::default()
                .buffer(uniform_buffers[i])
                .offset(0)
                .range(size_of::<UniformBufferObject>() as u64);
            let buffer_info = [descriptor_buffer_info];
            let descriptor_write = vk::WriteDescriptorSet::default()
                .dst_set(descriptor_sets[i])
                .dst_binding(0)
                .dst_array_element(0)
                .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                .buffer_info(&buffer_info);
            api.update_descriptor_sets(&[descriptor_write]);
        }

        let (pipeline, pipeline_layout, _) = api.create_graphics_pipeline(vertex_shader_module, fragment_shader_module, render_pass, descriptor_set_layout);
        let frame_buffers = api.create_frame_buffers(render_pass, swapchain_image_views.clone());
        let command_pool = api.create_command_pool(graphics_queue_index);
        let command_buffers = api.create_command_buffers(command_pool, swapchain_image_views.len());

        let image_available_semaphores = vec![api.create_semaphore(), api.create_semaphore()];
        let render_finished_semaphores = vec![api.create_semaphore(), api.create_semaphore()];
        let in_flight_fences = vec![api.create_fence(vk::FenceCreateFlags::SIGNALED), api.create_fence(vk::FenceCreateFlags::SIGNALED)];

        let mut uniform_buffer_object = UniformBufferObject::default();
        // uniform_buffer_object.proj = Matrix4::perspective(45.0, width as f32 / height as f32, 0.1, 100.0);

        uniform_buffer_object.proj = Matrix4::identity();


        Self {
            api,
            surface,
            width,
            height,
            graphics_queue,
            present_queue,
            swapchain,
            swapchain_image_views,
            vertex_shader_module,
            fragment_shader_module,
            render_pass,
            pipeline,
            pipeline_layout,
            descriptor_set_layout,
            frame_buffers,
            command_pool,
            command_buffers,
            image_available_semaphores,
            render_finished_semaphores,
            in_flight_fences,
            frame_index: 0,
            vertex_buffer: quad_vertex_buffer,
            vertex_memory: quad_vertex_memory,
            index_buffer: quad_index_buffer,
            index_memory: quad_index_memory,
            // world,
            // world_changed,
            // world_buffer,
            // world_memory,
            // world_staging_buffer,
            // world_staging_memory,
            texture,
            texture_changed: true,
            texture_image_buffer,
            texture_image_memory,
            texture_image_view,
            quad_vertices,
            quad_indices,
            world_changed: true,
            texture_sampler,
            descriptor_pool,
            uniform_buffers,
            uniform_memories,
            descriptor_sets,
            uniform_buffer_object,
        }
    }

    pub fn update_camera(&mut self, view_matrix: Matrix4) {
        self.uniform_buffer_object.view = view_matrix;
    }

    pub fn update_world(&mut self, world: &World) {
        self.world_changed = true;
    }

    fn prepare_image(&self) {
        let texture_bytes = self.texture.as_rgba8().expect("Failed to convert texture to rgba8").to_vec();
        let (texture_staging_buffer, texture_staging_memory) = self.api.create_staging_buffer(texture_bytes.len());

        self.api.transition_image_layout(self.texture_image_buffer, vk::ImageLayout::UNDEFINED, vk::ImageLayout::TRANSFER_DST_OPTIMAL, self.graphics_queue, self.command_pool);
        self.api.copy_to_device_memory(&texture_bytes, texture_staging_memory);
        self.api.exec_copy_buffer_to_image(texture_staging_buffer, self.texture_image_buffer, self.texture.width(), self.texture.height(), self.command_pool, self.graphics_queue);
        self.api.transition_image_layout(self.texture_image_buffer, vk::ImageLayout::TRANSFER_DST_OPTIMAL, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL, self.graphics_queue, self.command_pool);

        self.api.free_memory(texture_staging_memory);
        self.api.destroy_buffer(texture_staging_buffer);
    }

    pub fn init(&mut self) {
        self.prepare_image();
    }

    pub fn acquire_next_image(&self, image_available_semaphore: vk::Semaphore) {
        let image_index = self.api.acquire_next_image(self.swapchain, u64::MAX, image_available_semaphore, vk::Fence::null());
        assert_eq!(image_index, self.frame_index as u32);
        // println!("Image index / Frame index: {} / {}", image_index, self.frame_index as u32);
    }

    pub fn draw_frame(&mut self) {
        // determine next frame index


        // get frame resources
        let in_flight_fence = self.in_flight_fences[self.frame_index];
        let image_available_semaphore = self.image_available_semaphores[self.frame_index];
        let render_finished_semaphore = self.render_finished_semaphores[self.frame_index];
        let command_buffer = self.command_buffers[self.frame_index];
        let frame_buffer = self.frame_buffers[self.frame_index];
        let descriptor_sets = [self.descriptor_sets[self.frame_index]];

        // wait for previous frame to finish
        self.api.wait_for_fence(in_flight_fence);
        self.api.reset_fence(in_flight_fence);

        // begin rendering new frame
        self.acquire_next_image(image_available_semaphore);
        self.api.reset_command_buffer(command_buffer);
        self.api.begin_command_buffer(command_buffer, vk::CommandBufferUsageFlags::default());

        let staging_resources = if (self.world_changed) {
            let vertex_bytes = to_byte_slice(&self.quad_vertices);
            let (vertex_staging_buffer, vertex_staging_memory) = self.api.create_staging_buffer(vertex_bytes.len());
            self.api.copy_to_device_memory(vertex_bytes, vertex_staging_memory);
            self.api.cmd_copy_buffer(vertex_staging_buffer, self.vertex_buffer, vertex_bytes.len(), command_buffer);

            let index_bytes = to_byte_slice(&self.quad_indices);
            let (index_staging_buffer, index_staging_memory) = self.api.create_staging_buffer(index_bytes.len());
            self.api.copy_to_device_memory(index_bytes, index_staging_memory);
            self.api.cmd_copy_buffer(index_staging_buffer, self.index_buffer, index_bytes.len(), command_buffer);

            Some((
                vec![vertex_staging_buffer, index_staging_buffer],
                vec![vertex_staging_memory, index_staging_memory]
            ))
        } else {
            None
        };

        let uniform_bytes = to_bytes(&self.uniform_buffer_object);

        self.api.copy_to_device_memory(uniform_bytes, self.uniform_memories[self.frame_index]);

        self.api.cmd_bind_vertex_buffer(command_buffer, self.vertex_buffer);
        self.api.cmd_bind_index_buffer(command_buffer, self.index_buffer);
        self.api.cmd_bind_descriptor_sets(command_buffer, vk::PipelineBindPoint::GRAPHICS, self.pipeline_layout, &descriptor_sets);

        self.api.do_render_pass(command_buffer, self.pipeline, self.render_pass, frame_buffer, self.width as u32, self.height as u32, self.quad_indices.len() as u32);
        self.api.end_command_buffer(command_buffer);
        self.api.submit_command_buffer(self.graphics_queue, command_buffer, image_available_semaphore, render_finished_semaphore, in_flight_fence);
        self.api.present_frame(self.present_queue, self.swapchain, self.frame_index, render_finished_semaphore);

        if let Some((staging_buffer, staging_memory)) = staging_resources {
            for memory in staging_memory {
                self.api.free_memory(memory);
            }
            for buffer in staging_buffer {
                self.api.destroy_buffer(buffer);
            }
        }

        self.world_changed = false;
        self.frame_index = (self.frame_index + 1) % 2;
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            self.api.device_wait_idle();

            // self.api.free_memory(self.world_staging_memory);
            // self.api.destroy_buffer(self.world_staging_buffer);
            // self.api.free_memory(self.world_memory);
            // self.api.destroy_buffer(self.world_buffer);
            self.api.destroy_sampler(self.texture_sampler);
            self.api.destroy_image_view(self.texture_image_view);
            self.api.free_memory(self.texture_image_memory);
            self.api.destroy_image(self.texture_image_buffer);

            self.api.free_memory(self.vertex_memory);
            self.api.destroy_buffer(self.vertex_buffer);
            self.api.free_memory(self.index_memory);
            self.api.destroy_buffer(self.index_buffer);


            self.api.destroy_semaphores(self.image_available_semaphores.clone());
            self.api.destroy_semaphores(self.render_finished_semaphores.clone());
            self.api.destroy_fences(self.in_flight_fences.clone());

            self.api.destroy_command_pool(self.command_pool);
            self.api.destroy_framebuffers(self.frame_buffers.clone());
            self.api.destroy_render_pass(self.render_pass);

            for uniform_memory in self.uniform_memories.clone() {
                self.api.free_memory(uniform_memory);
            }
            for uniform_buffer in self.uniform_buffers.clone() {
                self.api.destroy_buffer(uniform_buffer);
            }

            self.api.destroy_descriptor_set_layout(self.descriptor_set_layout);
            self.api.destroy_descriptor_pool(self.descriptor_pool);
            self.api.destroy_pipeline_layout(self.pipeline_layout);
            self.api.destroy_pipeline(self.pipeline);

            self.api.destroy_shader_module(self.vertex_shader_module);
            self.api.destroy_shader_module(self.fragment_shader_module);
            self.api.destroy_image_views(self.swapchain_image_views.clone());
            self.api.destroy_swapchain(self.swapchain);
            self.api.destroy_surface(self.surface);
        }
    }
}

fn to_byte_slice<T: Sized>(slice: &[T]) -> &[u8] {
    unsafe {
        let pointer = slice.as_ptr() as *const u8;
        let size = slice.len() * size_of::<T>();
        from_raw_parts(pointer, size)
    }
}

fn to_bytes<T: Sized>(value: &T) -> &[u8] {
    unsafe {
        let pointer = value as *const T as *const u8;
        let size = size_of::<T>();
        from_raw_parts(pointer, size)
    }
}