use anyhow::{Context, Result};
use std::{
    fs::File,
    io::{BufWriter, Write},
};

#[derive(Clone)]
struct Vec3f {
    r: f32,
    b: f32,
    g: f32,
}

impl IntoIterator for Vec3f {
    type Item = f32;
    type IntoIter = Vec3fIterator;

    fn into_iter(self) -> Self::IntoIter {
        Vec3fIterator { pt: self, idx: 0 }
    }
}

struct Vec3fIterator {
    pt: Vec3f,
    idx: usize,
}

impl Iterator for Vec3fIterator {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        let ret = match self.idx {
            0 => Some(self.pt.r),
            1 => Some(self.pt.g),
            2 => Some(self.pt.b),
            _ => None,
        };

        self.idx += 1;
        ret
    }
}

fn main() -> Result<()> {
    let fpath = "out.ppm";
    let file = File::create(fpath).with_context(|| "Failed to create out.ppm")?;

    let height = 640;
    let width = 640;

    let mut framebuffer: Vec<Vec3f> = vec![
        Vec3f {
            r: 0.0,
            b: 0.0,
            g: 0.0
        };
        width * height
    ];

    // Populate framebuffer w/ gradient
    for y in 0..height {
        for x in 0..width {
            framebuffer[y * width + x] = Vec3f {
                r: y as f32 / height as f32,
                g: x as f32 / width as f32,
                b: 0.0,
            }
        }
    }

    println!("Framebuffer ready");

    // Save framebuffer to file
    let ppm_header = format!("P6\n{width} {height}\n255\n").into_bytes();

    let mut bufw = BufWriter::new(file);
    bufw.write(&ppm_header)
        .with_context(|| "Failed to write PPM header to file's bufwriter")?;

    for px in framebuffer {
        for x in px {
            // CHECK: wtf is this doing?
            let min = 1f32.min(x);
            let max = 0f32.max(min);
            let res = 255 * max as u8;
            bufw.write(&format!("{res}").into_bytes())
                .with_context(|| "Writing px color value failed")?;
        }
    }
    bufw.flush()
        .with_context(|| "Failed to flush file's bufwriter")?;

    println!("Framebuffer saved.");

    Ok(())
}
