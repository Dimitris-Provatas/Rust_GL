use crate::gl;

pub fn checker_board(pixels: &mut gl::Canvas) {
    const COLS: usize = 8;
    const ROWS: usize = 6;
    const CELL_WIDTH: usize = crate::WIDTH / COLS;
    const CELL_HEIGHT: usize = crate::HEIGHT / ROWS;

    const WHITE: usize = 0xFFFFFFFF;
    const BLACK: usize = 0x00000000;

    for y in 0..ROWS {
        for x in 0..COLS {
            let mut color = WHITE;

            if (x + y) % 2 == 0 {
                color = BLACK;
            }

            gl::fill_rect(
                pixels,
                crate::WIDTH,
                crate::HEIGHT,
                (x * CELL_WIDTH) as i32,
                (y * CELL_HEIGHT) as i32,
                CELL_WIDTH,
                CELL_HEIGHT,
                color,
            );
        }
    }
}

pub fn japan_flag(pixels: &mut gl::Canvas) {
    gl::fill_bg(pixels, crate::WIDTH, crate::HEIGHT, 0xFFFFFFFF);

    gl::fill_circle(
        pixels,
        crate::WIDTH,
        crate::HEIGHT,
        (crate::WIDTH / 2) as i32,
        (crate::HEIGHT / 2) as i32,
        150,
        0xFF0000FF,
    );
}

pub fn lines(pixels: &mut gl::Canvas) {
    // Red Lines
    gl::draw_line(
        pixels,
        crate::WIDTH,
        crate::HEIGHT,
        0,
        0,
        crate::WIDTH as i32 / 4,
        crate::HEIGHT as i32,
        0,
        0xFF0000FF,
    );
    gl::draw_line(
        pixels,
        crate::WIDTH,
        crate::HEIGHT,
        crate::WIDTH as i32,
        0,
        crate::WIDTH as i32 * 3 / 4,
        crate::HEIGHT as i32,
        0,
        0xFF0000FF,
    );

    // Green Lines
    gl::draw_line(
        pixels,
        crate::WIDTH,
        crate::HEIGHT,
        0,
        crate::HEIGHT as i32,
        crate::WIDTH as i32 / 2,
        0,
        0,
        0xFF00FF00,
    );
    gl::draw_line(
        pixels,
        crate::WIDTH,
        crate::HEIGHT,
        crate::WIDTH as i32,
        crate::HEIGHT as i32,
        crate::WIDTH as i32 / 2,
        0,
        0,
        0xFF00FF00,
    );

    // Blue Lines
    gl::draw_line(
        pixels,
        crate::WIDTH,
        crate::HEIGHT,
        crate::WIDTH as i32 / 2,
        0,
        crate::WIDTH as i32 / 2,
        crate::HEIGHT as i32,
        0,
        0xFFFF0000,
    );
    gl::draw_line(
        pixels,
        crate::WIDTH,
        crate::HEIGHT,
        0,
        crate::HEIGHT as i32 / 2,
        crate::WIDTH as i32,
        crate::HEIGHT as i32 / 2,
        0,
        0xFFFF0000,
    );

    // Yello Lines
    gl::draw_line(
        pixels,
        crate::WIDTH,
        crate::HEIGHT,
        0,
        0,
        crate::WIDTH as i32,
        crate::HEIGHT as i32,
        0,
        0xFF00FFFF,
    );
    gl::draw_line(
        pixels,
        crate::WIDTH,
        crate::HEIGHT,
        crate::WIDTH as i32,
        0,
        0,
        crate::HEIGHT as i32,
        0,
        0xFF00FFFF,
    );
}
