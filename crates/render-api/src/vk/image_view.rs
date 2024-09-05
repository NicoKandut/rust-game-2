use ash::vk;
use ash::vk::{ComponentSwizzle, Format, Image, ImageView};
use crate::vk::{VkRenderApi, COLOR_FORMAT};

/// Image Views
impl VkRenderApi {
    pub fn create_swapchain_images_views(&self, swapchain_images: Vec<Image>) -> Vec<ImageView> {
        let mut views = Vec::<ImageView>::with_capacity(swapchain_images.len());
        for image in swapchain_images {
            let image_view = self.create_image_view(image);
            views.push(image_view);
        }

        views
    }

    pub fn create_image_view(&self, image: Image) -> ImageView {
        let component_mapping = vk::ComponentMapping::default()
            .b(ComponentSwizzle::IDENTITY)
            .g(ComponentSwizzle::IDENTITY)
            .r(ComponentSwizzle::IDENTITY)
            .a(ComponentSwizzle::IDENTITY);
        let create_info = vk::ImageViewCreateInfo::default()
            .image(image)
            .view_type(vk::ImageViewType::TYPE_2D)
            .format(COLOR_FORMAT)
            .components(component_mapping)
            .subresource_range(vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            });
        unsafe {
            self.device.create_image_view(&create_info, None).expect("Failed to create image view!")
        }
    }

    pub fn destroy_image_view(&self, view: ImageView) {
        unsafe {
            self.device.destroy_image_view(view, None);
        }
    }

    pub fn destroy_image_views(&self, views: Vec<ImageView>) {
        for view in views {
            self.destroy_image_view(view);
        }
    }
}