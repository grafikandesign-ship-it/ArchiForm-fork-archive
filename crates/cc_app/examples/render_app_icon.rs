//! Regenerate the application PNGs and the multi-size Windows ICO from SVG.

use std::fs;
use std::path::PathBuf;

fn png_at(source: &str, size: u32) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let tree = resvg::usvg::Tree::from_str(source, &resvg::usvg::Options::default())?;
    let mut pixmap = resvg::tiny_skia::Pixmap::new(size, size).ok_or("invalid icon size")?;
    let scale = size as f32 / tree.size().width();
    resvg::render(&tree, resvg::tiny_skia::Transform::from_scale(scale, scale), &mut pixmap.as_mut());
    let mut bytes = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut bytes, size, size);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.write_header()?.write_image_data(pixmap.data())?;
    }
    Ok(bytes)
}

fn ico(images: &[(u32, Vec<u8>)]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let count = u16::try_from(images.len())?;
    let mut out = Vec::new();
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&count.to_le_bytes());
    let mut offset = 6 + images.len() * 16;
    for (size, image) in images {
        out.push(if *size == 256 { 0 } else { u8::try_from(*size)? });
        out.push(if *size == 256 { 0 } else { u8::try_from(*size)? });
        out.extend_from_slice(&[0, 0]);
        out.extend_from_slice(&1u16.to_le_bytes());
        out.extend_from_slice(&32u16.to_le_bytes());
        out.extend_from_slice(&u32::try_from(image.len())?.to_le_bytes());
        out.extend_from_slice(&u32::try_from(offset)?.to_le_bytes());
        offset += image.len();
    }
    for (_, image) in images {
        out.extend_from_slice(image);
    }
    Ok(out)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let icons = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/icons");
    let source = fs::read_to_string(icons.join("app-icon.svg"))?;
    fs::write(icons.join("app-icon.png"), png_at(&source, 512)?)?;
    fs::write(icons.join("app-icon-256.png"), png_at(&source, 256)?)?;
    fs::write(icons.join("app-icon-32.png"), png_at(&source, 32)?)?;
    fs::write(icons.join("app-icon-16.png"), png_at(&source, 16)?)?;
    let sizes = [16, 24, 32, 48, 64, 128, 256];
    let images = sizes
        .into_iter()
        .map(|size| Ok((size, png_at(&source, size)?)))
        .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?;
    fs::write(icons.join("app-icon.ico"), ico(&images)?)?;
    Ok(())
}
