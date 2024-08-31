use ash::vk;
use crate::vk::VkRenderApi;

impl VkRenderApi {
    pub fn create_sampler(&self) -> vk::Sampler {
        let sampler_create_info = vk::SamplerCreateInfo::default()
            .mag_filter(vk::Filter::default())
            .min_filter(vk::Filter::default())
            .address_mode_u(vk::SamplerAddressMode::CLAMP_TO_EDGE)
            .address_mode_v(vk::SamplerAddressMode::CLAMP_TO_EDGE)
            .address_mode_w(vk::SamplerAddressMode::CLAMP_TO_EDGE)
            .border_color(vk::BorderColor::FLOAT_OPAQUE_BLACK)
            .unnormalized_coordinates(false)
            .compare_enable(false)
            .compare_op(vk::CompareOp::ALWAYS)
            .anisotropy_enable(false)
            .max_anisotropy(2.0) // get from device properties
            .mipmap_mode(vk::SamplerMipmapMode::LINEAR)
            .mip_lod_bias(0.0)
            .min_lod(0.0)
            .max_lod(0.0);

        unsafe {
            self.device.create_sampler(&sampler_create_info, None)
                .expect("Failed to create sampler!")
        }
    }

    pub fn destroy_sampler(&self, sampler: vk::Sampler) {
        unsafe {
            self.device.destroy_sampler(sampler, None);
        }
    }
}