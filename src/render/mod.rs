pub mod font;

#[cfg(feature = "desktop_gl")]
pub mod glutin;
#[cfg(feature = "desktop_gl")]
pub use self::glutin::drawer::GlutinDrawer as drawer_impl;

#[cfg(feature = "raspberry_pi")]
pub mod pi;
#[cfg(feature = "raspberry_pi")]
pub use self::pi::drawer::PiDrawer as drawer_impl;

use crate::pos::Position;
use crate::pos::Rect;

use image::DynamicImage;

/// Represents a unsigned OpenGL color in Rust form.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    /// Creates a new copy, as Color is meant to remain immutable
    pub fn alpha(&self, a: u8) -> Self {
        let mut cloned = self.clone();
        cloned.a = a;
        cloned
    }

    pub fn new_4byte(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    pub fn new_3byte(r: u8, g: u8, b: u8) -> Self {
        Color::new_4byte(r, g, b, 255)
    }
}

/// Handles OpenGLES textures, and provides mechanisms for interacting/drawing on them
/// safely.
pub struct Texture {
    pub tex_data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Texture {
    pub fn draw_pixel(&mut self, color: &Color, x: usize, y: usize) {
        let starting_pos = (y * self.width + x) * 4;

        self.tex_data[starting_pos] = color.r;
        self.tex_data[starting_pos + 1] = color.g;
        self.tex_data[starting_pos + 2] = color.b;
        self.tex_data[starting_pos + 3] = color.a;
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    /// Creates a new Texture for drawing. This is only uploaded on demand.
    pub fn new(width: usize, height: usize) -> Self {
        Texture {
            tex_data: vec![0; width * height * 4],
            width,
            height,
        }
    }
}


/// The dimensions of a object
pub trait Dimensions {
    /// Returns the width of this object.
    fn get_width(&self) -> usize;

    /// Returns the height of this object.
    fn get_height(&self) -> usize;
}

/// Structures for rendering stuff to the screen
pub trait Drawer {
    type NativeTexture: Sized + Dimensions;

    /// Starts a particular rendering frame
    fn start(&mut self);

    /// Ends a frame, requesting for framebuffers to be finalised/etc
    fn end(&mut self);

    /// Clears the frame.
    /// transparent: If the frame should be cleared to alpha 0.
    fn clear(&mut self, transparent: bool);

    /// Enables blending of a texture/etc with the background, if this is
    ///  explicitly required.
    fn enable_blending(&mut self);

    /// Converts a texture to a native reference.
    fn convert_native_texture(&mut self, texture: Texture) -> Self::NativeTexture;

    /// Returns the width of the framebuffer.
    fn get_width(&self) -> usize;

    /// Returns the height of the framebuffer.
    fn get_height(&self) -> usize;

    /// Uses the specified image as a background. This is provided as several platforms
    /// have ways to accelerate this beyond OpenGL calls.
    fn set_background(&mut self, image: DynamicImage);

    /// Sets the screen brightness, if possible. Ignore call if not.
    fn set_brightness(&mut self, brightness: u8) -> ::std::io::Result<()>;

    /// Draws a texture to the screen, with a specified set of vertices to draw to, a UV
    /// to decode the image with, and a color to use as a base.
    fn draw_textured_vertices_colored_uv(
        &mut self,
        texture: &Self::NativeTexture,
        vertices: &[f32],
        colors: &[f32],
        uv: &[f32],
    );

    /// Draws a set of colored vertices to the screen, with a specified color array.
    fn draw_colored_vertices(&mut self, vertices: &[f32], colors: &[f32]);

    /// Returns the count of transitions that occured so far in this frame.
    fn get_transition_count(&self) -> usize;

    /// Draws a texture to the screen, with a specified set of vertices to draw to, and a color
    /// to use as a base.
    fn draw_textured_vertices_colored(
        &mut self,
        texture: &Self::NativeTexture,
        vertices: &[f32],
        colors: &[f32],
    ) {
        self.draw_textured_vertices_colored_uv(
            texture,
            vertices,
            colors,
            &[0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0],
        )
    }

    /// Draws a texture to the screen, with a specified set of vertices to draw to, and a
    /// default UV.
    fn draw_textured_vertices(&mut self, texture: &Self::NativeTexture, vertices: &[f32]) {
        self.draw_textured_vertices_colored(texture, vertices, &[1.0; 24])
    }

    /// Draws a texture to the screen, with the specified x/y coordinates (relative to screen size),
    ///  and a specified width/height.
    fn draw_texture_sized(&mut self, texture: &Self::NativeTexture, rect: &Rect, color: &Color) {
        let vertices = self.rect_to_vertices(rect);

        let mut colors: [f32; 24] = [0.0; 24];

        for i in 0..24 / 4 {
            colors[i * 4] = f32::from(color.r) / 255.0;
            colors[i * 4 + 1] = f32::from(color.g) / 255.0;
            colors[i * 4 + 2] = f32::from(color.b) / 255.0;
            colors[i * 4 + 3] = f32::from(color.a) / 255.0;
        }

        self.draw_textured_vertices_colored(texture, &vertices, &colors)
    }

    /// Draws a texture to the screen, with the specified x/y coordinates (relative to screen size),
    /// and the texture dimensions as width/height.
    fn draw_texture_colored(
        &mut self,
        texture: &Self::NativeTexture,
        pos: &Position,
        color: &Color,
    ) {
        let width = texture.get_width();
        let height = texture.get_height();

        self.draw_texture_sized(
            texture,
            &Rect::new_from_pos(pos, width as i32, height as i32),
            color,
        )
    }

    /// Draws a texture to the screen, with the specified x/y coordinates (relative to screen size),
    /// and the texture dimensions as width/height.
    fn draw_texture(&mut self, texture: &Self::NativeTexture, pos: &Position) {
        // TODO: Potentially dedicated shader for non colored?
        self.draw_texture_colored(texture, pos, &Color::new_4byte(255, 255, 255, 255))
    }

    /// Draws a colored rectangle to the screen, with a single color.
    fn draw_colored_rect(&mut self, rect: &Rect, color: &Color) {
        let vertices: [f32; 12] = self.rect_to_vertices(&rect);
        let mut colors: [f32; 24] = [0.0; 24];

        for i in 0..24 / 4 {
            colors[i * 4] = f32::from(color.r) / 255.0;
            colors[i * 4 + 1] = f32::from(color.g) / 255.0;
            colors[i * 4 + 2] = f32::from(color.b) / 255.0;
            colors[i * 4 + 3] = f32::from(color.a) / 255.0;
        }

        self.draw_colored_vertices(&vertices, &colors)
    }

    /// Converts a rectangle to 4 vertices
    fn rect_to_vertices(&self, rect: &Rect) -> [f32; 12] {
        // Translate to OpenGL coordinates
        let min_x = (rect.x as f32) / self.get_width() as f32 * 2.0 - 1.0;
        let max_x = ((rect.x + rect.width) as f32) / self.get_width() as f32 * 2.0 - 1.0;
        let min_y = (rect.y as f32) / self.get_height() as f32 * 2.0 - 1.0;
        let max_y = ((rect.y + rect.height) as f32) / self.get_height() as f32 * 2.0 - 1.0;

        // Generate vertex data
        // Inverted due to OpenGL perspective
        [
            // Vertex 1
            min_x, -min_y, min_x, -max_y, max_x, -max_y, // Vertex 2
            min_x, -min_y, max_x, -min_y, max_x, -max_y,
        ]
    }

    /// Creates a new instance of this drawer.
    ///
    /// All parameters are handled only if the target platform handles them.
    fn new(title : &str, width : u32, height : u32) -> Result<Self, String>
        where Self: std::marker::Sized;
}
