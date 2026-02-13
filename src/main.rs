pub mod app;
pub mod bvh;
pub mod camera;
pub mod colour;
pub mod geometry;
pub mod materials;
pub mod render;
pub mod scene;

use std::error::Error;
use winit::event_loop::{ControlFlow, EventLoop};

use crate::app::App;

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::with_user_event().build()?;
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut app = App::new();
    event_loop.run_app(&mut app)?;
    Ok(())
}
