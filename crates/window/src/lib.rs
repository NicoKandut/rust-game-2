use glfw::ClientApiHint::NoApi;
use glfw::WindowHint::ClientApi;
use glfw::WindowMode::Windowed;
use glfw::{init, GlfwReceiver, PWindow, WindowEvent};

pub struct Window {
    pub glfw: glfw::Glfw,
    pub p_window: PWindow,
    pub events: GlfwReceiver<(f64, WindowEvent)>,
}

impl Window {
    pub fn is_cursor_released(&self) -> bool {
        self.p_window.get_cursor_mode() == glfw::CursorMode::Normal
    }
}

impl Window {
    pub fn capture_cursor(&mut self) {
        self.p_window.set_cursor_mode(glfw::CursorMode::Disabled);
        self.p_window.set_cursor_pos_polling(true);
        self.p_window.set_key_polling(true);
    }

    pub fn release_cursor(&mut self) {
        self.p_window.set_cursor_mode(glfw::CursorMode::Normal);
        self.p_window.set_cursor_pos_polling(false);
        self.p_window.set_key_polling(false);
    }
}

impl Window {
    pub fn new(width: u32, height: u32) -> Self {
        let mut glfw = init(glfw::fail_on_errors).unwrap();
        assert!(glfw.vulkan_supported(), "Vulkan is not supported");
        glfw.window_hint(ClientApi(NoApi));


        let (mut p_window, events) = glfw
            .create_window(width, height, "Hello this is window", Windowed)
            .expect("Failed to create GLFW window.");


        p_window.set_resizable(true);

        p_window.set_mouse_button_polling(true);

        Self {
            glfw,
            p_window,
            events,
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {}
}

impl Window {
    pub fn get_required_extensions(&self) -> Option<Vec<String>> {
        self.glfw.get_required_instance_extensions()
    }
}