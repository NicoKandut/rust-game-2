use engine::Engine;
use renderer::Renderer;
use window::Window;

mod octree;
mod node;
mod material;
mod aabb;
mod octant;
mod camera;
mod ray;

fn main() {
    println!("MAIN > Started");

    let window = Window::new();
    println!("MAIN > Window created");

    let mut engine = Engine::new();
    println!("MAIN > Engine created");

    let renderer = Renderer::new(&window.p_window);

    let max_delay = std::time::Duration::from_millis(1000 / 60);

    let mut last_time = std::time::Instant::now();
    while !window.p_window.should_close() {
        'loop_events: while last_time.elapsed() < max_delay {
            if let Some((f, event)) = window.events.receive() {
                engine.accept(event);
                last_time = std::time::Instant::now();
                break 'loop_events;
            }
        }

        println!("MAIN > New frame after {:?}", last_time.elapsed());
        renderer.render();


        last_time = std::time::Instant::now();

    }
}
