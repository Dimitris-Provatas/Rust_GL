use examples::examples;

mod examples;
mod glib;
mod store;

pub const WIDTH: usize = 800;
pub const HEIGHT: usize = 600;

static mut PIXELS: glib::Canvas = [0; WIDTH * HEIGHT];

const BG_COLOR: usize = 0xFF282828;
const FG_COLOR: usize = 0xFF0000FF;

fn main() {
    unsafe {
        examples(&mut PIXELS);
    }

    0;
}
