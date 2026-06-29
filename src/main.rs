use anyhow::{Context, Result};
use std::{
    fs::File,
    io::{BufWriter, Write},
};

mod geometry;

use geometry::{Sphere, Vec3f};

fn main() -> Result<()> {
    let fpath = "out.ppm";
    let file = File::create(fpath).with_context(|| "Failed to create out.ppm")?;

    let height = 640;
    let width = 640;

    let mut framebuffer: Vec<Vec3f> = vec![Vec3f::new(0f32, 0f32, 0f32); width * height];
    let sphere = Sphere::new(Vec3f::new(0f32, 0f32, 0f32), 4f32);

    // Populate framebuffer w/ gradient
    // TODO: render sphere to framebuffer
    for y in 0..height {
        for x in 0..width {
            framebuffer[y * width + x] =
                Vec3f::new(y as f32 / height as f32, x as f32 / width as f32, 0.0)
        }
    }

    println!("Framebuffer ready");

    // Show sphere on screen

    // Save framebuffer to file
    let ppm_header = format!("P6\n{width} {height}\n255\n").into_bytes();

    let mut bufw = BufWriter::new(file);
    bufw.write(&ppm_header)
        .with_context(|| "Failed to write PPM header to file's bufwriter")?;

    for px in framebuffer {
        for x in px {
            let clamped = x.clamp(0f32, 1f32);
            let byte = (255f32 * clamped) as u8;
            bufw.write_all(&[byte])
                .with_context(|| "Writing px color value failed")?;
        }
    }
    bufw.flush()
        .with_context(|| "Failed to flush file's bufwriter")?;

    println!("Framebuffer saved.");

    Ok(())
}
