//! Provides mechanisms for receiving input from the screen.

#[cfg(feature = "desktop_gl")]
mod glutin;
#[cfg(feature = "desktop_gl")]
pub use self::glutin::GlutinInput as input_impl;

#[cfg(feature = "raspberry_pi")]
mod pi;
#[cfg(feature = "raspberry_pi")]
pub use self::pi::PiInput as input_impl;

use crate::render::Drawer;

/// Handles basic input
pub trait Input {
    type Window: Drawer;

    /// Updates input
    fn update(&mut self, drawer: &mut Self::Window);

    /// Checks to see if the mouse/pointer is down
    fn is_mouse_down(&self) -> bool;

    /// Returns the current mouse position in a (x, y) tuple.
    fn get_mouse_pos(&self) -> (usize, usize);

    /// Checks to see if execution should be continued
    fn do_continue(&self) -> bool;

    /// Creates a new Input instance.
    fn new() -> Self;
}
