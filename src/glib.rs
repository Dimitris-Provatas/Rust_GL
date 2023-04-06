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
    // Stupid Borrow Checker, why do you have to be memory safe?
    let mut x1 = point1_x;
    let mut y1 = point1_y;
    let mut x2 = point2_x;
    let mut y2 = point2_y;
    let mut x3 = point3_x;
    let mut y3 = point3_y;

    // Hard-coded Bubble-Sort because we efficient
    if y1 > y2 {
        swap(&mut y1, &mut y2);
        swap(&mut x1, &mut x2);
    }
    if y2 > y3 {
        swap(&mut y2, &mut y3);
        swap(&mut x2, &mut x3);
    }
    if y1 > y2 {
        swap(&mut y1, &mut y2);
        swap(&mut x1, &mut x2);
    }

    // Top half
    let dx12 = x2 - x1;
    let dy12 = y2 - y1;
    let dx13 = x3 - x1;
    let dy13 = y3 - y1;

    for y in RangeInclusive::new(y1, y2) {
        if y >= 0 && y < canvas_height as i32 {
            let mut s1 = if dy12 != 0 {
                (y - y1) * dx12 / dy12 + x1
            } else {
                x1
            };
            let mut s2 = if dy13 != 0 {
                (y - y1) * dx13 / dy13 + x1
            } else {
                x1
            };

            if s1 > s2 {
                swap(&mut s1, &mut s2);
            }

            for x in RangeInclusive::new(s1, s2) {
                if x >= 0 && x < canvas_width as i32 {
                    pixels[(y * canvas_width as i32 + x) as usize] = color;
                }
            }
        }
    }

    // Bottom half
    let dx32 = x2 - x3;
    let dy32 = y2 - y3;
    let dx31 = x1 - x3;
    let dy31 = y1 - y3;

    for y in RangeInclusive::new(y2, y3) {
        if y >= 0 && y < canvas_height as i32 {
            let mut s1 = if dy32 != 0 {
                (y - y3) * dx32 / dy32 + x3
            } else {
                x3
            };
            let mut s2 = if dy31 != 0 {
                (y - y3) * dx31 / dy31 + x3
            } else {
                x3
            };

            if s1 > s2 {
                swap(&mut s1, &mut s2);
            }

            for x in RangeInclusive::new(s1, s2) {
                if x >= 0 && x < canvas_width as i32 {
                    pixels[(y * canvas_width as i32 + x) as usize] = color;
                }
            }
        }
    }
}

// TODO: Change all nested ifs in loops to break or continue (e.g. x>=0 -> x < 0 continue | x<canvas_width -> x > canvas_width break )
