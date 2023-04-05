mod examples;
mod gl;
mod store;

pub const WIDTH: usize = 800;
pub const HEIGHT: usize = 600;

static mut PIXELS: gl::Canvas = [0; WIDTH * HEIGHT];

const BG_COLOR: usize = 0xFF282828;
const FG_COLOR: usize = 0xFF0000FF;

fn checker_example() {
    unsafe {
        examples::checker_board(&mut PIXELS);

        let _ = store::save_ppm(
            &mut PIXELS,
            WIDTH,
            HEIGHT,
            "outputs/checker_example.ppm".to_string(),
        );
    }
}

fn japan_example() {
    unsafe {
        examples::japan_flag(&mut PIXELS);

        let _ = store::save_ppm(
            &mut PIXELS,
            WIDTH,
            HEIGHT,
            "outputs/japan_flag_example.ppm".to_string(),
        );
    }
}

fn lines_example() {
    unsafe {
        examples::lines(&mut PIXELS);

        let _ = store::save_ppm(
            &mut PIXELS,
            WIDTH,
            HEIGHT,
            "outputs/lines_example.ppm".to_string(),
        );
    }
}

fn main() {
    unsafe {
        gl::fill_bg(&mut PIXELS, WIDTH, HEIGHT, BG_COLOR);

        let rect_w: usize = 200;
        let rect_h: usize = 120;
        gl::fill_rect(
            &mut PIXELS,
            WIDTH,
            HEIGHT,
            WIDTH as i32 / 2 - rect_w as i32 / 2,
            HEIGHT as i32 / 2 - rect_h as i32 / 2,
            rect_w,
            rect_h,
            0xFF000000,
        );

        gl::fill_circle(
            &mut PIXELS,
            WIDTH,
            HEIGHT,
            WIDTH as i32 / 2 + rect_w as i32 / 2,
            HEIGHT as i32 / 2 + rect_h as i32 / 2,
            50,
            0xFF00FF00,
        );
        gl::fill_circle(
            &mut PIXELS,
            WIDTH,
            HEIGHT,
            WIDTH as i32 / 2 - rect_w as i32 / 2,
            HEIGHT as i32 / 2 + rect_h as i32 / 2,
            50,
            0xFFFF0000,
        );

        gl::draw_line(
            &mut PIXELS,
            WIDTH,
            HEIGHT,
            0,
            0,
            WIDTH as i32 / 2 - rect_w as i32 / 2,
            HEIGHT as i32 / 2 - rect_h as i32 / 2,
            0,
            0xFF0000FF,
        );
        gl::draw_line(
            &mut PIXELS,
            WIDTH,
            HEIGHT,
            WIDTH as i32,
            0,
            WIDTH as i32 / 2 + rect_w as i32 / 2,
            HEIGHT as i32 / 2 - rect_h as i32 / 2,
            0,
            0xFF0000FF,
        );

        let _ = store::save_ppm(&mut PIXELS, WIDTH, HEIGHT, "outputs/main.ppm".to_string());
    }

    0;
}
