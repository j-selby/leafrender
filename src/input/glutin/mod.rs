use glutin;

use crate::input::Input;

use crate::render::glutin::drawer::GlutinDrawer;

/// Implements a basic input abstraction for Glutin.
pub struct GlutinInput {
    mouse_down: bool,
    mouse_x: usize,
    mouse_y: usize,
    running: bool,
}

impl Input for GlutinInput {
    type Window = GlutinDrawer;

    fn update(&mut self, window: &mut Self::Window) {
        let events = &mut window.events_loop;
        let window = &window.gl_window;

        events.poll_events(|event| {
            if let glutin::Event::WindowEvent { event, .. } = event {
                match event {
                    glutin::WindowEvent::CloseRequested => self.running = false,
                    glutin::WindowEvent::Resized(size) => window.resize(size.to_physical(1.0)),
                    glutin::WindowEvent::MouseInput { state, .. } => {
                        self.mouse_down = state == glutin::ElementState::Pressed;
                    }
                    glutin::WindowEvent::CursorMoved { position, .. } => {
                        let (x, y): (i32, i32) = position.into();
                        self.mouse_x = x as usize;
                        self.mouse_y = y as usize;
                    }
                    _ => (),
                }
            }
        });
    }

    fn is_mouse_down(&self) -> bool {
        self.mouse_down
    }

    fn get_mouse_pos(&self) -> (usize, usize) {
        (self.mouse_x, self.mouse_y)
    }

    fn do_continue(&self) -> bool {
        self.running
    }

    fn new() -> Self {
        GlutinInput {
            mouse_down: false,
            mouse_x: 0,
            mouse_y: 0,
            running: true,
        }
    }
}
