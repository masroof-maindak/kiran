use std::{fs::File, io::Write, path::Path};

struct Point3D;

// Saves the output image as a .ppm file
fn save_image(
    fpath: &Path,
    width: usize,
    height: usize,
    body: Vec<Point3D>,
) -> std::io::Result<()> {
    let mut file = File::create(fpath)?;

    let header = format!("P6\n{width} {height}\n255\n").into_bytes();
    file.write(&header)?;

    // TODO: write body.
    for _px in body {}

    Ok(())
}

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    save_image(Path::new("Out.ppm"), 1920, 1080)?;

    Ok(())
}
