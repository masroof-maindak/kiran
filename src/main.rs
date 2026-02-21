use std::{fs::File, io::Write, path::Path};

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

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let fpath = "Out.ppm";
    // save_image(Path::new("Out.ppm"), 1920, 1080)?;

    let mut file = File::create(fpath)?;

    let width = 1920;
    let height = 1080;
    let mut framebuffer: Vec<Vec3f> = vec![
        Vec3f {
            r: 0.0,
            b: 0.0,
            g: 0.0
        };
        width * height
    ];

    // Populate framebuffer w/ gradient
    for i in 0..=height {
        for j in 0..=width {
            framebuffer[i + j * width] = Vec3f {
                r: j as f32 / height as f32,
                b: 0.0,
                g: i as f32 / width as f32,
            }
        }
    }

    // Save framebuffer to file
    let header = format!("P6\n{width} {height}\n255\n").into_bytes();
    file.write(&header)?;
    for px in framebuffer {
        for x in px {}
    }

    Ok(())
}
