use std::mem::swap;
use std::ops::RangeInclusive;

pub type Canvas = [usize; crate::WIDTH * crate::HEIGHT];

pub fn fill_bg(pixels: &mut Canvas, width: usize, height: usize, color: usize) {
    for i in 0..width * height {
        pixels[i] = color;
    }
}

pub fn fill_rect(
    pixels: &mut Canvas,
    canvas_width: usize,
    canvas_height: usize,
    rect_pos_x: i32,
    rect_pos_y: i32,
    rect_width: usize,
    rect_height: usize,
    color: usize,
) {
    for dy in 0..rect_height as i32 {
        let y = rect_pos_y + dy;

        if y >= 0 && y < canvas_height as i32 {
            for dx in 0..rect_width as i32 {
                let x = rect_pos_x + dx;

                if x >= 0 && x < canvas_width as i32 {
                    pixels[(y * canvas_width as i32 + x) as usize] = color;
                }
            }
        }
    }
}

pub fn fill_circle(
    pixels: &mut Canvas,
    canvas_width: usize,
    canvas_height: usize,
    center_x: i32,
    center_y: i32,
    radius: usize,
    color: usize,
) {
    let x1 = center_x - radius as i32;
    let y1 = center_y - radius as i32;
    let x2 = center_x + radius as i32;
    let y2 = center_y + radius as i32;

    for y in y1..y2 {
        if y >= 0 && y < canvas_height as i32 {
            for x in x1..x2 {
                if x >= 0 && x < canvas_width as i32 {
                    let dx = x - center_x;
                    let dy = y - center_y;

                    if (dx * dx) as i32 + (dy * dy) as i32 <= (radius * radius) as i32 {
                        pixels[(y * canvas_width as i32 + x) as usize] = color;
                    }
                }
            }
        }
    }
}

pub fn draw_line(
    pixels: &mut Canvas,
    canvas_width: usize,
    canvas_height: usize,
    start_x: i32,
    start_y: i32,
    end_x: i32,
    end_y: i32,
    thiccness: usize,
    color: usize,
) {
    // Stupid Borrow Checker, why do you have to be memory safe?
    let mut x1 = start_x;
    let mut x2 = end_x;
    let mut y1 = start_y;
    let mut y2 = end_y;

    /*
     * y = k * x + c // line equation
     * =>
     *
     * y1 = k * x1 + c
     * y2 = k * x2 + c
     * =>
     *
     * y1 - k * x1 = c ->
     * y2          = k * x2 + y1 - k * x1
     * =>
     *
     * y1 - k * x1 = c
     * y2          = k * (x2 - x1) + y1
     * =>
     *
     * y1 - k * x1           = c
     * (y2 - y1) / (x2 - x1) = k -> dy / dx = k
     */
    let dx: i32 = end_x - start_x;
    let dy: i32 = end_y - start_y;

    if dx != 0 {
        let c = start_y - dy * start_x as i32 / dx;

        if start_x > end_x {
            swap(&mut x1, &mut x2);
        }

        for x in x1..x2 {
            if x >= 0 && x < canvas_width as i32 {
                let mut cy = dy * x / dx + c;
                let mut ny = dy * (x + 1) / dx + c;

                if cy > ny {
                    swap(&mut cy, &mut ny);
                }

                for y in RangeInclusive::new(cy, ny) {
                    if y >= 0 && y < canvas_height as i32 {
                        pixels[(y * canvas_width as i32 + x) as usize] = color;
                    }
                }
            }
        }
    } else {
        // vertical line -> dx == 0
        if start_x >= 0 && start_x < canvas_width as i32 {
            if start_y > end_y {
                swap(&mut y1, &mut y2);
            }

            for y in y1..y2 {
                if y >= 0 && y < canvas_height as i32 {
                    pixels[(y * canvas_width as i32 + start_x) as usize] = color;
                }
            }
        }
    }
}

pub fn fill_triangle(
    pixels: &mut Canvas,
    canvas_width: usize,
    canvas_height: usize,
    point1_x: i32,
    point1_y: i32,
    point2_x: i32,
    point2_y: i32,
    point3_x: i32,
    point3_y: i32,
    color: usize,
) {
}
