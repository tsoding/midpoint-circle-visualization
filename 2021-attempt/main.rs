use std::fs::File;
use std::io;
use std::io::{Write, BufWriter};

fn save_as_ppm(file_path: &str, pixels: &[u32], width: usize, height: usize) -> io::Result<()> {
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

fn draw_hollow_circle(pixels: &mut [u32], width: usize, height: usize, radius: usize, foreground: u32, background: u32) {
    pixels.fill(background);

    // TODO: integer subpixel computations in draw_hollow_circle()

    let w = width as f32;
    let h = height as f32;
    let r = radius as f32;
    let cx = w / 2.0;
    let cy = h / 2.0;
    let mut x = 0.0;
    let mut y = r - 0.5;

    while x <= y {
        let px = x + cx;
        let py = y + cy;
        if (0.0..w).contains(&px) && (0.0..h).contains(&py) {
            let dx = px as usize;
            let dy = py as usize;
            assert!(width == height);

            pixels[dy * width + dx] = foreground;
            pixels[dx * width + dy] = foreground;

            pixels[(height - dy) * width + dx] = foreground;
            pixels[dx * width + (height - dy)] = foreground;

            pixels[dy * width + (width - dx)] = foreground;
            pixels[(width - dx) * width + dy] = foreground;

            pixels[(height - dy) * width + (width - dx)] = foreground;
            pixels[(width - dx) * width + (height - dy)] = foreground;
        }

        x += 1.0;
        if x*x + y*y > r*r {
            y -= 1.0;
        }
    }
}

fn main() {
    const WIDTH: usize = 256;
    const HEIGHT: usize = 256;
    const FOREGROUND: u32 = 0xFF00FF;
    const BACKGROUND: u32 = 0x000000;
    let mut pixels = [0u32; WIDTH * HEIGHT];

    pixels.fill(0x00FF00);
    stripes_pattern(&mut pixels, WIDTH, HEIGHT, WIDTH / 16, FOREGROUND, BACKGROUND);
    save_as_ppm("stripes.ppm", &pixels, WIDTH, HEIGHT).unwrap();

    pixels.fill(0x00FF00);
    checker_pattern(&mut pixels, WIDTH, HEIGHT, WIDTH / 16, FOREGROUND, BACKGROUND);
    save_as_ppm("checker.ppm", &pixels, WIDTH, HEIGHT).unwrap();

    pixels.fill(0x00FF00);
    fill_solid_circle(&mut pixels, WIDTH, HEIGHT, WIDTH / 3, FOREGROUND, BACKGROUND);
    save_as_ppm("solid.ppm", &pixels, WIDTH, HEIGHT).unwrap();

    pixels.fill(0x00FF00);
    draw_hollow_circle(&mut pixels, WIDTH, HEIGHT, WIDTH / 3, FOREGROUND, BACKGROUND);
    save_as_ppm("hollow.ppm", &pixels, WIDTH, HEIGHT).unwrap();
}
