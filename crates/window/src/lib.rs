use glfw::{GlfwReceiver, init, PWindow, WindowEvent};
use glfw::ClientApiHint::NoApi;
use glfw::WindowHint::ClientApi;
use glfw::WindowMode::Windowed;

pub struct Window {
    pub glfw: glfw::Glfw,
    pub p_window: PWindow,
    pub events: GlfwReceiver<(f64, WindowEvent)>,
}

impl Window {
    pub fn new() -> Self {
        let mut glfw = init(glfw::fail_on_errors).unwrap();
        glfw.window_hint(ClientApi(NoApi));

        let (mut p_window, events) = glfw
            .create_window(600, 300, "Hello this is window", Windowed)
            .expect("Failed to create GLFW window.");

        p_window.set_key_polling(true);
        p_window.set_resizable(true);

        Self {
            glfw,
            p_window,
            events,
        }
    }
}