use std::{num::NonZeroU32, rc::Rc};

use softbuffer::{Context, Surface};
use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, OwnedDisplayHandle},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

use crate::{render::Renderer, scene::InputMovement};

pub struct App {
    window: Option<Rc<Window>>,
    surface: Option<Surface<OwnedDisplayHandle, Rc<Window>>>,
    renderer: Option<Renderer>,
}

impl App {
    pub fn new() -> Self {
        Self {
            window: None,
            surface: None,
            renderer: None,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_inner_size(winit::dpi::LogicalSize::new(1920.0, 1080.0));
        let window = event_loop.create_window(window_attributes).unwrap();
        let window = Rc::new(window);
        let context = Context::new(event_loop.owned_display_handle()).unwrap();
        let surface = Surface::new(&context, window.clone()).unwrap();
        let size = window.inner_size();
        self.window = Some(window);
        self.surface = Some(surface);
        self.renderer = Some(Renderer::new(size));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let renderer = match &mut self.renderer {
            Some(canvas) => canvas,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                renderer.render(self.surface.as_mut().unwrap());
            }
            WindowEvent::Resized(size) => {
                self.surface
                    .as_mut()
                    .unwrap()
                    .resize(
                        NonZeroU32::new(size.width).unwrap(),
                        NonZeroU32::new(size.height).unwrap(),
                    )
                    .unwrap();
                renderer.resize(size);
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(key),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                match key {
                    KeyCode::KeyA => renderer.scene.move_camera(InputMovement::Left),
                    KeyCode::KeyD => renderer.scene.move_camera(InputMovement::Right),
                    KeyCode::KeyW => renderer.scene.move_camera(InputMovement::Forward),
                    KeyCode::KeyS => renderer.scene.move_camera(InputMovement::Backward),
                    KeyCode::KeyQ => renderer.scene.move_camera(InputMovement::Up),
                    KeyCode::KeyE => renderer.scene.move_camera(InputMovement::Down),
                    KeyCode::KeyI => renderer.scene.move_camera(InputMovement::RotateUp),
                    KeyCode::KeyJ => renderer.scene.move_camera(InputMovement::RotateLeft),
                    KeyCode::KeyK => renderer.scene.move_camera(InputMovement::RotateDown),
                    KeyCode::KeyL => renderer.scene.move_camera(InputMovement::RotateRight),
                    _ => (),
                };
                self.window.as_mut().unwrap().request_redraw();
            }
            _ => {}
        }
    }

    // fn device_event(
    //     &mut self,
    //     _event_loop: &ActiveEventLoop,
    //     _device_id: winit::event::DeviceId,
    //     event: DeviceEvent,
    // ) {
    //     let renderer = match &mut self.renderer {
    //         Some(canvas) => canvas,
    //         None => return,
    //     };

    //     match event {
    //         DeviceEvent::MouseMotion { delta } => {
    //             renderer
    //                 .scene
    //                 .move_camera(InputMovement::RotateX(0.003 * delta.0));
    //             renderer
    //                 .scene
    //                 .move_camera(InputMovement::RotateY(0.003 * delta.1));
    //             self.window.as_mut().unwrap().request_redraw();
    //         }
    //         _ => (),
    //     }
    // }
}
