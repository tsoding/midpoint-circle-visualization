use std::fs::File;
use std::io;
use std::io::{Write, BufWriter};

fn save_as_ppm(file_path: &str, pixels: &[u32], width: usize, height: usize) -> io::Result<()>
{
    let mut file = BufWriter::with_capacity(width * height * 3, File::create(file_path)?);
    write!(file, "P6\n{} {} 255\n", width, height)?;
    for y in 0..height {
        for x in 0..width {
            let pixel = pixels[y * width + x];
            let color = [((pixel >> 8 * 2) & 0xFF) as u8,
                         ((pixel >> 8 * 1) & 0xFF) as u8,
                         ((pixel >> 8 * 0) & 0xFF) as u8];
            file.write(&color)?;
        }
    }
    println!("Saved {}", file_path);
    Ok(())
}

fn stripes_pattern(pixels: &mut [u32], width: usize, height: usize, tile_size: usize, foreground: u32, background: u32) {
    for y in 0..height {
        for x in 0..width {
            pixels[y * width + x] = if ((x + y) / tile_size) % 2 == 0 {
                background
            } else {
                foreground
            };
        }
    }
}

fn checker_pattern(pixels: &mut [u32], width: usize, height: usize, tile_size: usize, foreground: u32, background: u32) {
    for y in 0..height {
        for x in 0..width {
            pixels[y * width + x] = if (x / tile_size + y / tile_size) % 2 == 0 {
                background
            } else {
                foreground
            };
        }
    }
}

fn fill_solid_circle(pixels: &mut [u32], width: usize, height: usize, radius: usize, foreground: u32, background: u32)
{
    let cx = width as i32;
    let cy = height as i32;
    let r = radius as i32 * 2;
    for y in 0..height {
        for x in 0..width {
            let dx = cx - x as i32 * 2 - 1;
            let dy = cy - y as i32 * 2 - 1;

            pixels[y * width + x] = if dx*dx + dy*dy <= r*r {
                foreground
            } else {
                background
            };
        }
    }
}

fn draw_hollow_circle(_pixels: &mut [u32], _width: usize, _height: usize, _radius: usize, _foreground: u32, _background: u32) {
    todo!();
}

fn main() {
    const WIDTH: usize = 16;
    const HEIGHT: usize = 16;
    const FOREGROUND: u32 = 0xFF00FF;
    const BACKGROUND: u32 = 0x000000;
    let mut pixels = [0u32; WIDTH * HEIGHT];

    pixels.fill(0x00FF00);
    stripes_pattern(&mut pixels, WIDTH, HEIGHT, 32, FOREGROUND, BACKGROUND);
    save_as_ppm("stripes.ppm", &pixels, WIDTH, HEIGHT);

    pixels.fill(0x00FF00);
    checker_pattern(&mut pixels, WIDTH, HEIGHT, 32, FOREGROUND, BACKGROUND);
    save_as_ppm("checker.ppm", &pixels, WIDTH, HEIGHT);

    pixels.fill(0x00FF00);
    fill_solid_circle(&mut pixels, WIDTH, HEIGHT, WIDTH / 2, FOREGROUND, BACKGROUND);
    save_as_ppm("solid.ppm", &pixels, WIDTH, HEIGHT);

    pixels.fill(0x00FF00);
    draw_hollow_circle(&mut pixels, WIDTH, HEIGHT, HEIGHT / 2, FOREGROUND, BACKGROUND);
    save_as_ppm("hollow.ppm", &pixels, WIDTH, HEIGHT);
}
