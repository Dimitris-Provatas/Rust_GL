use std::fs::File;
use std::io;
use std::io::Write;

use crate::glib::Canvas;

pub fn save_ppm(
    pixels: &mut Canvas,
    width: usize,
    height: usize,
    file_path: String,
) -> Result<(), io::Error> {
    let mut file = File::create(file_path)?;

    let mut file_str = format!("P3 {} {} 255\n", width, height);
    for i in 0..width * height {
        let pixel = pixels[i];

        file_str.push_str(&format!(
            "{} {} {}\n",
            (pixel >> (8 * 0)) & 0xFF,
            (pixel >> (8 * 1)) & 0xFF,
            (pixel >> (8 * 2)) & 0xFF,
        ));
    }

    write!(file, "{}", file_str)?;
    Ok(())
}
