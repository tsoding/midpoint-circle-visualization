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

fn lerp(a: f32, b: f32, p: f32) -> f32 {
    a + (b - a) * p
}

// TODO: it would be probably better to do the color blending in integers since we working directly with 32 bit color representation
fn blend_pixels_gamma_corrected(background: u32, foreground: u32, p: f32) -> u32 {
    let br = (background >> (8 * 2)) & 0xFF;
    let fr = (foreground >> (8 * 2)) & 0xFF;
    let r = lerp((br * br) as f32, (fr * fr) as f32, p).sqrt() as u32;

    let bg = (background >> (8 * 1)) & 0xFF;
    let fg = (foreground >> (8 * 1)) & 0xFF;
    let g = lerp((bg * bg) as f32, (fg * fg) as f32, p).sqrt() as u32;

    let bb = (background >> (8 * 0)) & 0xFF;
    let fb = (foreground >> (8 * 0)) & 0xFF;
    let b = lerp((bb * bb) as f32, (fb * fb) as f32, p).sqrt() as u32;

    (r << (8 * 2)) | (g << (8 * 1)) | (b << (8 * 0))
}

fn blend_pixels_naively(background: u32, foreground: u32, p: f32) -> u32 {
    let br = (background >> (8 * 2)) & 0xFF;
    let fr = (foreground >> (8 * 2)) & 0xFF;
    let r = lerp(br as f32, fr as f32, p) as u32;

    let bg = (background >> (8 * 1)) & 0xFF;
    let fg = (foreground >> (8 * 1)) & 0xFF;
    let g = lerp(bg as f32, fg as f32, p) as u32;

    let bb = (background >> (8 * 0)) & 0xFF;
    let fb = (foreground >> (8 * 0)) & 0xFF;
    let b = lerp(bb as f32, fb as f32, p) as u32;

    (r << (8 * 2)) | (g << (8 * 1)) | (b << (8 * 0))
}

fn fill_solid_aa_circle<Blender>(pixels: &mut [u32], width: usize, height: usize, radius: usize, foreground: u32, background: u32, blend_pixels: Blender) where Blender: Fn(u32, u32, f32) -> u32 {
    let cx = width as f32 * 0.5;
    let cy = height as f32 * 0.5;
    let r = radius as f32;

    const AA_RES: usize = 3;
    const AA_STEP: f32 = 1.0 / (AA_RES + 1) as f32;

    for y in 0..height {
        for x in 0..width {
            let mut aa_count = 0;
            for aay in 0..AA_RES {
                for aax in 0..AA_RES {
                    let px = x as f32 + AA_STEP + aax as f32 * AA_STEP;
                    let py = y as f32 + AA_STEP + aay as f32 * AA_STEP;
                    let dx = cx - px;
                    let dy = cy - py;
                    if dx*dx + dy*dy <= r*r {
                        aa_count += 1;
                    }
                }
            }
            let p = aa_count as f32 / (AA_RES * AA_RES) as f32;

            pixels[y*width + x] = blend_pixels(background, foreground, p);
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
    const WIDTH: usize = 32;
    const HEIGHT: usize = 32;
    const RADIUS: usize = WIDTH / 3;
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
    fill_solid_circle(&mut pixels, WIDTH, HEIGHT, RADIUS, FOREGROUND, BACKGROUND);
    save_as_ppm("solid.ppm", &pixels, WIDTH, HEIGHT).unwrap();

    pixels.fill(0x00FF00);
    fill_solid_aa_circle(&mut pixels, WIDTH, HEIGHT, RADIUS, FOREGROUND, BACKGROUND, blend_pixels_naively);
    save_as_ppm("solid-aa-naively.ppm", &pixels, WIDTH, HEIGHT).unwrap();

    pixels.fill(0x00FF00);
    fill_solid_aa_circle(&mut pixels, WIDTH, HEIGHT, RADIUS, FOREGROUND, BACKGROUND, blend_pixels_gamma_corrected);
    save_as_ppm("solid-aa-gamma-corrected.ppm", &pixels, WIDTH, HEIGHT).unwrap();

    pixels.fill(0x00FF00);
    draw_hollow_circle(&mut pixels, WIDTH, HEIGHT, RADIUS, FOREGROUND, BACKGROUND);
    save_as_ppm("hollow.ppm", &pixels, WIDTH, HEIGHT).unwrap();
}
