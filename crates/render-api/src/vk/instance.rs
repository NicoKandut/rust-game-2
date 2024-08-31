use std::ffi::CString;
use ash::vk;

pub fn create_instance(entry: &ash::Entry) -> ash::Instance {
    let game_name = CString::new("Game").unwrap();
    let engine_name = CString::new("Nico Engine").unwrap();
    let application_info = vk::ApplicationInfo::default()
        .application_name(game_name.as_c_str())
        .application_version(vk::make_api_version(0, 1, 0, 0))
        .engine_name(engine_name.as_c_str())
        .engine_version(vk::make_api_version(0, 1, 0, 0))
        .api_version(vk::API_VERSION_1_3);
    let extension_names = vec![
        ash::ext::debug_utils::NAME.as_ptr(),
        ash::khr::surface::NAME.as_ptr(),
        ash::khr::win32_surface::NAME.as_ptr(),
    ];
    let validation_layer_name = CString::new("VK_LAYER_KHRONOS_validation".to_string()).unwrap();
    let layers = vec![validation_layer_name.as_ptr()];
    let instance_create_info = vk::InstanceCreateInfo::default()
        .enabled_extension_names(&extension_names)
        .enabled_layer_names(&layers)
        .application_info(&application_info);

    unsafe {
        let instance = entry
            .create_instance(&instance_create_info, None)
            .unwrap();
        instance
    }
}