use crate::glib;
use crate::store;

pub fn examples(pixels: &mut glib::Canvas) {
    checker_board(pixels);

    japan_flag(pixels);

    lines(pixels);

    triangles(pixels);

    logo(pixels);

    alpha_blending(pixels);
}

fn checker_board(pixels: &mut glib::Canvas) {
    const COLS: i32 = 8;
    const ROWS: i32 = 6;
    const CELL_WIDTH: i32 = crate::WIDTH as i32 / COLS;
    const CELL_HEIGHT: i32 = crate::HEIGHT as i32 / ROWS;

    const WHITE: usize = 0xFFFFFFFF;
    const BLACK: usize = 0x00000000;

    for y in 0..ROWS {
        for x in 0..COLS {
            let mut color = WHITE;

            if (x + y) % 2 == 0 {
                color = BLACK;
            }

            glib::fill_rect(
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

    let _ = store::save_ppm(
        pixels,
        crate::WIDTH,
        crate::HEIGHT,
        "outputs/checker_example.ppm".to_string(),
    );
}

fn japan_flag(pixels: &mut glib::Canvas) {
    glib::fill_bg(pixels, crate::WIDTH, crate::HEIGHT, 0xFFFFFFFF);

    glib::fill_circle(
        pixels,
        crate::WIDTH,
        crate::HEIGHT,
        (crate::WIDTH / 2) as i32,
        (crate::HEIGHT / 2) as i32,
        150,
        0xFF0000FF,
    );

    let _ = store::save_ppm(
        pixels,
        crate::WIDTH,
        crate::HEIGHT,
        "outputs/japan_flag_example.ppm".to_string(),
    );
}

fn lines(pixels: &mut glib::Canvas) {
    glib::fill_bg(pixels, crate::WIDTH, crate::HEIGHT, crate::BG_COLOR);

    // Red Lines
    glib::draw_line(
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
    glib::draw_line(
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
    glib::draw_line(
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
    glib::draw_line(
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
    glib::draw_line(
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
    glib::draw_line(
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
    glib::draw_line(
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
    glib::draw_line(
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

    let _ = store::save_ppm(
        pixels,
        crate::WIDTH,
        crate::HEIGHT,
        "outputs/lines_example.ppm".to_string(),
    );
}

fn logo(pixels: &mut glib::Canvas) {
    glib::fill_bg(pixels, crate::WIDTH, crate::HEIGHT, crate::BG_COLOR);

    let rect_w: i32 = 200;
    let rect_h: i32 = 120;
    glib::fill_rect(
        pixels,
        crate::WIDTH,
        crate::HEIGHT,
        crate::WIDTH as i32 / 2 - rect_w as i32 / 2,
        crate::HEIGHT as i32 / 2 - rect_h as i32 / 2,
        rect_w,
        rect_h,
        0xFF000000,
    );

    glib::fill_circle(
        pixels,
        crate::WIDTH,
        crate::HEIGHT,
        crate::WIDTH as i32 / 2 + rect_w as i32 / 2,
        crate::HEIGHT as i32 / 2 + rect_h as i32 / 2,
        50,
        0xFF00FF00,
    );
    glib::fill_circle(
        pixels,
        crate::WIDTH,
        crate::HEIGHT,
        crate::WIDTH as i32 / 2 - rect_w as i32 / 2,
        crate::HEIGHT as i32 / 2 + rect_h as i32 / 2,
        50,
        0xFFFF0000,
    );

    glib::draw_line(
        pixels,
        crate::WIDTH,
        crate::HEIGHT,
        0,
        0,
        crate::WIDTH as i32 / 2 - rect_w as i32 / 2,
        crate::HEIGHT as i32 / 2 - rect_h as i32 / 2,
        0,
        0xFF0000FF,
    );
    glib::draw_line(
        pixels,
        crate::WIDTH,
        crate::HEIGHT,
        crate::WIDTH as i32,
        0,
        crate::WIDTH as i32 / 2 + rect_w as i32 / 2,
        crate::HEIGHT as i32 / 2 - rect_h as i32 / 2,
        0,
        0xFF0000FF,
    );

    let _ = store::save_ppm(
        pixels,
        crate::WIDTH,
        crate::HEIGHT,
        "outputs/logo.ppm".to_string(),
    );
}

fn triangles(pixels: &mut glib::Canvas) {
    glib::fill_bg(pixels, crate::WIDTH, crate::HEIGHT, crate::BG_COLOR);

    let p_color = 0xFF00FF00;
    let p_radius = 5;

    // Triangle Red
    {
        let x1 = crate::WIDTH as i32 / 2;
        let y1 = crate::HEIGHT as i32 / 8;
        let x2 = crate::WIDTH as i32 / 8;
        let y2 = crate::HEIGHT as i32 / 2;
        let x3 = crate::WIDTH as i32 * 7 / 8;
        let y3 = crate::HEIGHT as i32 * 7 / 8;

        glib::fill_circle(
            pixels,
            crate::WIDTH,
            crate::HEIGHT,
            x1,
            y1,
            p_radius,
            p_color,
        );
        glib::fill_circle(
            pixels,
            crate::WIDTH,
            crate::HEIGHT,
            x2,
            y2,
            p_radius,
            p_color,
        );
        glib::fill_circle(
            pixels,
            crate::WIDTH,
            crate::HEIGHT,
            x3,
            y3,
            p_radius,
            p_color,
        );

        glib::fill_triangle(
            pixels,
            crate::WIDTH,
            crate::HEIGHT,
            x1,
            y1,
            x2,
            y2,
            x3,
            y3,
            crate::FG_COLOR,
        );
    }

    // Triangle Blue
    {
        let x1 = crate::WIDTH as i32 / 2;
        let y1 = crate::HEIGHT as i32 / 4;
        let x2 = crate::WIDTH as i32 / 4;
        let y2 = crate::HEIGHT as i32 / 2;
        let x3 = crate::WIDTH as i32 * 3 / 4;
        let y3 = crate::HEIGHT as i32 / 2;

        glib::fill_circle(
            pixels,
            crate::WIDTH,
            crate::HEIGHT,
            x1,
            y1,
            p_radius,
            p_color,
        );
        glib::fill_circle(
            pixels,
            crate::WIDTH,
            crate::HEIGHT,
            x2,
            y2,
            p_radius,
            p_color,
        );
        glib::fill_circle(
            pixels,
            crate::WIDTH,
            crate::HEIGHT,
            x3,
            y3,
            p_radius,
            p_color,
        );

        glib::fill_triangle(
            pixels,
            crate::WIDTH,
            crate::HEIGHT,
            x1,
            y1,
            x2,
            y2,
            x3,
            y3,
            0xFFFF0000,
        );
    }

    // Triangle Yellow
    {
        let x1 = crate::WIDTH as i32 / 8;
        let y1 = crate::HEIGHT as i32 / 8;
        let x2 = crate::WIDTH as i32 / 8;
        let y2 = crate::HEIGHT as i32 * 3 / 8;
        let x3 = crate::WIDTH as i32 * 3 / 8;
        let y3 = crate::HEIGHT as i32 * 3 / 8;

        glib::fill_circle(
            pixels,
            crate::WIDTH,
            crate::HEIGHT,
            x1,
            y1,
            p_radius,
            p_color,
        );
        glib::fill_circle(
            pixels,
            crate::WIDTH,
            crate::HEIGHT,
            x2,
            y2,
            p_radius,
            p_color,
        );
        glib::fill_circle(
            pixels,
            crate::WIDTH,
            crate::HEIGHT,
            x3,
            y3,
            p_radius,
            p_color,
        );

        glib::fill_triangle(
            pixels,
            crate::WIDTH,
            crate::HEIGHT,
            x1,
            y1,
            x2,
            y2,
            x3,
            y3,
            0xFF00FFFF,
        );
    }

    let _ = store::save_ppm(
        pixels,
        crate::WIDTH,
        crate::HEIGHT,
        "outputs/triangles_example.ppm".to_string(),
    );
}

fn alpha_blending(pixels: &mut glib::Canvas) {
    glib::fill_bg(pixels, crate::WIDTH, crate::HEIGHT, crate::BG_COLOR);

    glib::fill_rect(
        pixels,
        crate::WIDTH,
        crate::HEIGHT,
        0,
        0,
        crate::WIDTH as i32 * 3 / 4,
        crate::HEIGHT as i32 * 3 / 4,
        0xFF0000FF,
    );
    glib::fill_rect(
        pixels,
        crate::WIDTH,
        crate::HEIGHT,
        crate::WIDTH as i32,
        crate::HEIGHT as i32,
        -(crate::WIDTH as i32) * 3 / 8,
        -(crate::HEIGHT as i32) * 3 / 8,
        0xCCFF0000,
    );

    glib::fill_circle(
        pixels,
        crate::WIDTH,
        crate::HEIGHT,
        crate::WIDTH as i32 * 3 / 4,
        crate::HEIGHT as i32 * 3 / 8,
        175,
        0xAA00FF00,
    );

    glib::fill_triangle(
        pixels,
        crate::WIDTH,
        crate::HEIGHT,
        crate::WIDTH as i32 / 2,
        crate::HEIGHT as i32 / 8,
        crate::WIDTH as i32 / 8,
        crate::HEIGHT as i32 * 7 / 8,
        crate::WIDTH as i32 * 23 / 32,
        crate::HEIGHT as i32 * 3 / 4,
        0x8800FFFF,
    );

    let _ = store::save_ppm(
        pixels,
        crate::WIDTH,
        crate::HEIGHT,
        "outputs/alpha_blend_example.ppm".to_string(),
    );
}
