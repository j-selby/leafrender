extern crate image;
extern crate rusttype;

extern crate libc;

#[cfg(feature = "raspberry_pi")]
extern crate egl;
#[cfg(feature = "raspberry_pi")]
extern crate opengles;
#[cfg(feature = "raspberry_pi")]
extern crate videocore;
#[cfg(feature = "raspberry_pi")]
extern crate evdev;

#[cfg(feature = "desktop_gl")]
extern crate glutin;
#[cfg(feature = "desktop_gl")]
extern crate gl;

pub mod pos;

pub mod render;
pub use crate::render::drawer_impl as PlatformDrawer;

pub mod input;
pub use crate::input::input_impl as PlatformInput;

use crate::input::Input;
use crate::render::Drawer;
use crate::pos::Rect;
use crate::render::Color;

fn main() {
    let mut drawer = PlatformDrawer::new("LeafRender Test", 640, 480)
        .expect("Failed to create drawer");
    let mut input = PlatformInput::new();

    while {
        input.update(&mut drawer);
        input.do_continue()
    } {
        drawer.start();
        drawer.clear(false);

        drawer.draw_colored_rect(
            &Rect {
                x: 100,
                y: 100,
                width: 50,
                height: 50,
            },
            &Color {
                r: 255,
                g: 0,
                b: 0,
                a: 255,
            }
        );

        drawer.end();
    }
}
