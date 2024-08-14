use glfw::WindowEvent;

pub struct Engine {

}


impl Engine {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn accept(&mut self, event: WindowEvent) {
        println!("Engine accepted {event:?}");
    }
}