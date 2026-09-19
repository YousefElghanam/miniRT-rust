use winit::event_loop;

use crate::app;
use crate::scene::Scene;

pub fn render(scene: Scene) {
    let event_loop = event_loop::EventLoop::new().unwrap();
    let mut application = app::create(scene);
    event_loop.run_app(&mut application).unwrap();
}
