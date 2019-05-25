extern crate image;
extern crate leafrender;

use leafrender::input::Input;
use leafrender::pos::Position;
use leafrender::pos::Rect;
use leafrender::render::font::FontCache;
use leafrender::render::Color;
use leafrender::render::Drawer;

use leafrender::PlatformDrawer;
use leafrender::PlatformInput;

fn main() {
    let mut drawer =
        PlatformDrawer::new("LeafRender Test", 640, 480).expect("Failed to create drawer");
    let mut input = PlatformInput::new();

    let mut font = FontCache::from_bytes(include_bytes!("../res/Lato-Regular.ttf"))
        .expect("Unable to load font");

    let image = image::load_from_memory(include_bytes!("../res/ferris.png"))
        .expect("Failed to load image")
        .to_rgba();

    let image = drawer.convert_image(&image);

    let mut step = 0;
    while {
        input.update(&mut drawer);
        input.do_continue()
    } {
        step += 1;
        step %= 360;

        drawer.start();
        drawer.clear(false);
        drawer.enable_blending();

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
            },
        );

        font.draw(
            "Hello, World!",
            &Color::new_3byte(255, 255, 255),
            24,
            &Position { x: 50, y: 50 },
            &mut drawer,
        );

        let ferris_x_pos = (f64::from(step) * std::f64::consts::PI / 180.0).sin() * 100.0 + 150.0;
        let ferris_y_pos = (f64::from(step) * std::f64::consts::PI / 180.0).cos() * 100.0 + 150.0;

        drawer.draw_texture_sized(
            &image,
            &Rect {
                x: ferris_x_pos as _,
                y: ferris_y_pos as _,
                width: 120,
                height: 80,
            },
            &Color {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            },
        );

        drawer.end();
    }
}
