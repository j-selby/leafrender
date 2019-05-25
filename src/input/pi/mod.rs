use crate::input::Input;

use crate::render::pi::drawer::PiDrawer;

use evdev;

/// Implements a basic input mechanism for the Pi through evdev.
pub struct PiInput {
    devices: Vec<evdev::Device>,
    mouse_x: usize,
    mouse_y: usize,
    mouse_down: bool,
}

impl Input for PiInput {
    type Window = PiDrawer;

    /// Updates input
    fn update(&mut self, _: &mut Self::Window) {
        let mut input = Vec::new();
        for device in &mut self.devices {
            for evt in device.events_no_sync().unwrap() {
                input.push(evt);
            }
        }

        let mut touched = false;

        for input in input {
            if input._type == 3 {
                touched = true;
                break;
            }
        }

        self.mouse_down = touched;
    }

    /// Checks to see if the mouse/pointer is down
    fn is_mouse_down(&self) -> bool {
        self.mouse_down
    }

    fn get_mouse_pos(&self) -> (usize, usize) {
        unimplemented!()
    }

    // No way of telling this
    fn do_continue(&self) -> bool {
        true
    }

    fn new() -> Self {
        let devices = evdev::enumerate();

        for device in &devices {
            println!("Found input device: {:?}", device.name());
        }

        PiInput {
            devices,
            mouse_x: 0,
            mouse_y: 0,
            mouse_down: false,
        }
    }
}
