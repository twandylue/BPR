use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn save_as_ppm(
    file_path: &Path,
    pixels: &mut [u32],
    height: usize,
    width: usize,
) -> std::io::Result<()> {
    let mut file = BufWriter::new(File::create(file_path)?);
    file.write(format!("P6\n{width} {height} 255\n").as_str().as_bytes())?;
    for y in 0..height {
        for x in 0..width {
            let pixel = pixels[y * width + x];
            let color = [
                ((pixel >> 8 * 2) & 0xFF) as u8,
                ((pixel >> 8 * 1) & 0xFF) as u8,
                ((pixel >> 8 * 0) & 0xFF) as u8,
            ];
            file.write(&color)?;
        }
    }

    println!("Save {file_path}", file_path = file_path.display());

    Ok(())
}

fn fill_rectangle(pixels: &mut [u32], foreground: u32) {
    pixels.fill(foreground);
}

fn striped_pattern(
    pixels: &mut [u32],
    height: usize,
    width: usize,
    foreground: u32,
    background: u32,
    tile_size: usize,
) {
    for y in 0..height {
        for x in 0..width {
            pixels[y * width + x] = if ((x + y) / tile_size) % 2 == 0 {
                background
            } else {
                foreground
            }
        }
    }
}

fn checker_pattern(
    pixels: &mut [u32],
    height: usize,
    width: usize,
    foreground: u32,
    background: u32,
    tile_size: usize,
) {
    for y in 0..height {
        for x in 0..width {
            pixels[y * width + x] = if (x / tile_size + y / tile_size) % 2 == 0 {
                background
            } else {
                foreground
            }
        }
    }
}

fn fill_solid_circle(
    pixels: &mut [u32],
    radius: usize,
    height: usize,
    width: usize,
    foreground: u32,
    background: u32,
) {
    let r = (radius * 2) as i32;
    let cx = ((width * 2) / 2) as i32;
    let cy = ((height * 2) / 2) as i32;
    for y in 0..height {
        for x in 0..width {
            let dx = cx - (x as i32 * 2 + 1);
            let dy = cy - (y as i32 * 2 + 1);

            pixels[y * width + x] = if dx * dx + dy * dy <= r * r {
                foreground
            } else {
                background
            }
        }
    }
}

fn halo_circle(
    pixels: &mut [u32],
    height: usize,
    width: usize,
    foreground: u32,
    background: u32,
) {
    todo!()
}

fn main() {
    const HEIGHT: usize = 16;
    const WIDTH: usize = 16;
    const BACKGROUND: u32 = 0x000000;
    const FOREGROUND: u32 = 0xFF0000;
    let mut pixels = [0_u32; WIDTH * HEIGHT];

    fill_rectangle(&mut pixels, FOREGROUND);
    save_as_ppm(Path::new("rectangle.ppm"), &mut pixels, HEIGHT, WIDTH)
        .unwrap_or_else(|err| eprintln!("ERROR: could not save as ppm file: {err}"));

    striped_pattern(&mut pixels, HEIGHT, WIDTH, FOREGROUND, BACKGROUND, 32);
    save_as_ppm(
        Path::new("stripped_pattern.ppm"),
        &mut pixels,
        HEIGHT,
        WIDTH,
    )
    .unwrap_or_else(|err| eprintln!("ERROR: could not save as ppm file: {err}"));

    checker_pattern(&mut pixels, HEIGHT, WIDTH, FOREGROUND, BACKGROUND, 32);
    save_as_ppm(Path::new("checker_pattern.ppm"), &mut pixels, HEIGHT, WIDTH)
        .unwrap_or_else(|err| eprintln!("ERROR: could not save as ppm file: {err}"));

    fill_solid_circle(
        &mut pixels,
        WIDTH / 2,
        HEIGHT,
        WIDTH,
        FOREGROUND,
        BACKGROUND,
    );
    save_as_ppm(Path::new("solid_circle.ppm"), &mut pixels, HEIGHT, WIDTH)
        .unwrap_or_else(|err| eprintln!("ERROR: could not save as ppm file: {err}"));

    // TODO:
    // halo_circle(&mut pixels, HEIGHT, WIDTH, FOREGROUND, BACKGROUND);
    // save_as_ppm(Path::new("halo_circle.ppm"), &mut pixels, HEIGHT, WIDTH)
    //     .unwrap_or_else(|err| eprintln!("ERROR: could not save as ppm file: {err}"));
}
