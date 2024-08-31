use crate::vk::VkRenderApi;
use ash::vk;

impl VkRenderApi {
    pub fn create_frame_buffers(&self, render_pass: vk::RenderPass, image_views: Vec<vk::ImageView>) -> Vec<vk::Framebuffer> {
        let mut frame_buffers = Vec::<vk::Framebuffer>::with_capacity(image_views.len());

        for image_view in image_views {
            let attachments = [image_view];
            let frame_buffer_create_info = vk::FramebufferCreateInfo::default()
                .render_pass(render_pass)
                .attachments(&attachments)
                .width(600)
                .height(300)
                .layers(1);
            let frame_buffer = unsafe {
                self.device.create_framebuffer(&frame_buffer_create_info, None)
                    .expect("Failed to create frame buffer!")
            };
            frame_buffers.push(frame_buffer);
        }

        frame_buffers
    }

    pub fn destroy_framebuffers(&self, framebuffers: Vec<vk::Framebuffer>) {
        for framebuffer in framebuffers {
            unsafe {
                self.device.destroy_framebuffer(framebuffer, None);
            }
        }
    }
}