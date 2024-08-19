use std::mem::size_of;
use std::sync::Arc;

use ash::vk::{Pipeline, PipelineLayout, Queue, SurfaceKHR};
use glfw::PWindow;

use render_api::vk::VkRenderApi;
use shader::get_rt_shader;
use world::octree::Octree;

pub struct Renderer {
    api: VkRenderApi,
    surface: SurfaceKHR,
    queue: Queue,
    pipeline: Pipeline,
    pipeline_layout: PipelineLayout,
    world: Arc<Octree>,
    world_changed,
}

impl Renderer {
    pub fn new(p_window: &PWindow) -> Self {
        let api = VkRenderApi::default();

        let surface = api.create_surface(p_window);
        let queue = api.create_queue();

        let world = Arc::new(Octree::default());
        let world_changed = true;
        let world_size = world.size_in_bytes();

        let (world_staging_buffer, world_staging_memory) = api.create_staging_buffer(world_size);
        let (world_buffer, world_memory) = api.create_shader_storage_buffer(world_size);

        let (width, height) = p_window.get_size();
        let (image_buffer, image_memory) = api.create_image(width as u32, height as u32);

        let compute_shader_code = get_rt_shader();
        let compute_shader_module = api.create_shader_module(&compute_shader_code);
        let (pipeline, pipeline_layout) = api.create_compute_pipeline(compute_shader_module);

        Self {
            api,
            surface,
            queue,
            pipeline,
            pipeline_layout,
            world,
            world_changed,
        }
    }

    pub fn update_world(&mut self, world: Arc<Octree>) {
        self.world = world;
    }

    pub fn render(&self) {
        // println!("Rendering...");
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            self.api.destroy_pipeline(self.pipeline)
        }
    }
}