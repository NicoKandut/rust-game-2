use std::mem::size_of;
use std::sync::Arc;
use engine::Engine;
use renderer::Renderer;
use window::Window;
use world::node::Node;
use world::octree::Octree;

mod camera;
mod ray;

fn main() {
    println!("MAIN > Started");

    assert_eq!(size_of::<Node>(), 88);

    let window = Window::new();
    println!("MAIN > Window created");

    let mut engine = Engine::new();
    println!("MAIN > Engine created");

    let mut renderer = Renderer::new(&window.p_window);

    let max_delay = std::time::Duration::from_millis(1000 / 60);

    let world = Arc::new(Octree::default());
    let mut world_changed = false;

    let mut last_time = std::time::Instant::now();
    while !window.p_window.should_close() {
        'loop_events: while last_time.elapsed() < max_delay {
            if let Some((f, event)) = window.events.receive() {
                engine.accept(event);
                last_time = std::time::Instant::now();
                break 'loop_events;
            }
        }

        if(world_changed) {
            // println!("MAIN > Updating world");
            renderer.update_world(world.clone())
        }

        // println!("MAIN > New frame after {:?}", last_time.elapsed());
        renderer.render();


        last_time = std::time::Instant::now();

    }
}
